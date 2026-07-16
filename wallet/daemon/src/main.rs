use std::str::FromStr;
use std::sync::Arc;

use clap::Parser;
use tokio::sync::{Mutex, oneshot};
use tonic::{Request, Response, Status, transport::Server};
use velkar_addresses::Address;
use velkar_consensus_core::network::{NetworkId, NetworkType};
use velkar_wallet_core::api::{
    AccountsActivateRequest, AccountsCreateNewAddressRequest, AccountsEnumerateRequest, AccountsGetUtxosRequest,
    AccountsPskbBroadcastRequest, AccountsPskbCreateRequest, AccountsPskbSignRequest, AccountsSelectRequest, ConnectRequest,
    NewAddressKind, WalletApi, WalletOpenRequest,
};
use velkar_wallet_core::deterministic::AccountId;
use velkar_wallet_core::prelude::{Fees, PaymentDestination, PaymentOutput};
use velkar_wallet_core::rpc::Resolver;
use velkar_wallet_core::tx::PaymentOutputs;
use velkar_wallet_core::wallet::Wallet;
use velkar_wallet_keys::secret::Secret;

pub mod proto {
    tonic::include_proto!("velkarwalletd");
}

use proto::fee_policy::FeePolicy as ProtoFeePolicy;
use proto::velkarwalletd_server::{Velkarwalletd, VelkarwalletdServer};
use proto::*;

#[derive(Parser, Debug)]
#[command(name = "velkar-walletd", version, about = "Velkar wallet gRPC daemon")]
struct Args {
    #[arg(long, default_value = "127.0.0.1:28110")]
    listen: String,

    #[arg(long, default_value = "ws://127.0.0.1:17110")]
    wrpc: String,

    #[arg(long, default_value = "mainnet")]
    network: String,

    #[arg(long, default_value = "velkar")]
    wallet: String,

    #[arg(long)]
    password: String,

    #[arg(long)]
    payment_secret: Option<String>,

    #[arg(long)]
    account_name: Option<String>,

    #[arg(long, default_value_t = false)]
    allow_unsynced: bool,
}

#[derive(Clone)]
struct AppState {
    wallet: Arc<Wallet>,
    wallet_secret: Secret,
    payment_secret: Option<Secret>,
    account_name: Option<String>,
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl AppState {
    async fn descriptors(&self) -> Result<Vec<velkar_wallet_core::account::descriptor::AccountDescriptor>, Status> {
        self.wallet
            .clone()
            .accounts_enumerate_call(AccountsEnumerateRequest {})
            .await
            .map(|r| r.account_descriptors)
            .map_err(status_from_error)
    }

    async fn ensure_selected_account(&self) -> Result<AccountId, Status> {
        let descriptors = self.descriptors().await?;
        let selected = if let Some(account_name) = self.account_name.as_deref() {
            descriptors
                .iter()
                .find(|d| d.account_name.as_deref() == Some(account_name))
                .ok_or_else(|| Status::not_found(format!("account '{account_name}' not found")))?
        } else {
            descriptors.first().ok_or_else(|| Status::not_found("wallet has no accounts"))?
        };

        self.wallet
            .clone()
            .accounts_select_call(AccountsSelectRequest { account_id: Some(selected.account_id) })
            .await
            .map_err(status_from_error)?;

        Ok(selected.account_id)
    }

    async fn resolve_account_from_addresses(&self, from: &[String]) -> Result<AccountId, Status> {
        if from.is_empty() {
            return self.ensure_selected_account().await;
        }

        let descriptors = self.descriptors().await?;
        for descriptor in descriptors {
            let mut owned = Vec::new();
            if let Some(address) = &descriptor.receive_address {
                owned.push(address.to_string());
            }
            if let Some(address) = &descriptor.change_address {
                owned.push(address.to_string());
            }
            if let Some(addresses) = &descriptor.addresses {
                owned.extend(addresses.iter().map(ToString::to_string));
            }

            if from.iter().any(|candidate| owned.iter().any(|owned_addr| owned_addr == candidate)) {
                self.wallet
                    .clone()
                    .accounts_select_call(AccountsSelectRequest { account_id: Some(descriptor.account_id) })
                    .await
                    .map_err(status_from_error)?;
                return Ok(descriptor.account_id);
            }
        }

        self.ensure_selected_account().await
    }

    fn wallet_secret_from_request(&self, password: &str) -> Result<Secret, Status> {
        if !password.is_empty() && password.trim() != self.wallet_secret.as_str().map_err(status_from_error)? {
            return Err(Status::permission_denied("invalid wallet password"));
        }

        Ok(self.wallet_secret.clone())
    }
}

struct WalletdService {
    state: AppState,
}

#[tonic::async_trait]
impl Velkarwalletd for WalletdService {
    async fn get_balance(&self, _request: Request<GetBalanceRequest>) -> Result<Response<GetBalanceResponse>, Status> {
        let descriptors = self.state.descriptors().await?;
        let mut available = 0u64;
        let mut pending = 0u64;
        let mut address_balances = Vec::new();

        for descriptor in descriptors {
            if let Some(balance) = descriptor.balance.as_ref() {
                available = available.saturating_add(balance.mature);
                pending = pending.saturating_add(balance.pending);
            }

            if let Some(address) = descriptor.receive_address.as_ref() {
                let balance = descriptor.balance.clone().unwrap_or_default();
                address_balances.push(AddressBalances {
                    address: address.to_string(),
                    available: balance.mature,
                    pending: balance.pending,
                });
            }
        }

        Ok(Response::new(GetBalanceResponse { available, pending, address_balances }))
    }

    async fn get_external_spendable_utx_os(
        &self,
        request: Request<GetExternalSpendableUtxOsRequest>,
    ) -> Result<Response<GetExternalSpendableUtxOsResponse>, Status> {
        let request = request.into_inner();
        let account_id = self.state.resolve_account_from_addresses(std::slice::from_ref(&request.address)).await?;
        let address = Address::try_from(request.address.as_str()).map_err(status_from_error)?;
        let response = self
            .state
            .wallet
            .clone()
            .accounts_get_utxos_call(AccountsGetUtxosRequest {
                account_id,
                addresses: Some(vec![address]),
                min_amount_sompi: None,
            })
            .await
            .map_err(status_from_error)?;

        let entries = response
            .utxos
            .into_iter()
            .map(|utxo| UtxosByAddressesEntry {
                address: utxo.address.map(|a| a.to_string()).unwrap_or_default(),
                outpoint: Some(Outpoint {
                    transaction_id: utxo.outpoint.transaction_id.to_string(),
                    index: utxo.outpoint.index,
                }),
                utxo_entry: Some(UtxoEntry {
                    amount: utxo.amount,
                    script_public_key: Some(ScriptPublicKey {
                        version: utxo.script_public_key.version.into(),
                        script_public_key: utxo.script_public_key.script_as_hex(),
                    }),
                    block_daa_score: utxo.block_daa_score,
                    is_coinbase: utxo.is_coinbase,
                }),
            })
            .collect();

        Ok(Response::new(GetExternalSpendableUtxOsResponse { entries }))
    }

    async fn create_unsigned_transactions(
        &self,
        request: Request<CreateUnsignedTransactionsRequest>,
    ) -> Result<Response<CreateUnsignedTransactionsResponse>, Status> {
        let request = request.into_inner();
        if request.is_send_all {
            return Err(Status::unimplemented("isSendAll is not supported"));
        }

        let account_id = self.state.resolve_account_from_addresses(&request.from).await?;
        let destination_address = Address::try_from(request.address.as_str()).map_err(status_from_error)?;
        let (fee_rate, priority_fee_sompi) = resolve_fee_policy(request.fee_policy.as_ref());

        let pskb = self
            .state
            .wallet
            .clone()
            .accounts_pskb_create_call(AccountsPskbCreateRequest {
                account_id,
                wallet_secret: self.state.wallet_secret.clone(),
                payment_secret: self.state.payment_secret.clone(),
                destination: PaymentDestination::from(PaymentOutputs::from((destination_address, request.amount))),
                fee_rate,
                priority_fee_sompi,
                payload: None,
            })
            .await
            .map_err(status_from_error)?;

        Ok(Response::new(CreateUnsignedTransactionsResponse { unsigned_transactions: vec![pskb.pskb.into_bytes()] }))
    }

    async fn show_addresses(&self, _request: Request<ShowAddressesRequest>) -> Result<Response<ShowAddressesResponse>, Status> {
        let descriptors = self.state.descriptors().await?;
        let mut addresses = Vec::new();
        for descriptor in descriptors {
            if let Some(address) = descriptor.receive_address {
                addresses.push(address.to_string());
            }
            if let Some(address) = descriptor.change_address {
                addresses.push(address.to_string());
            }
            if let Some(extra) = descriptor.addresses {
                addresses.extend(extra.into_iter().map(|a| a.to_string()));
            }
        }

        addresses.sort();
        addresses.dedup();

        Ok(Response::new(ShowAddressesResponse { address: addresses }))
    }

    async fn new_address(&self, _request: Request<NewAddressRequest>) -> Result<Response<NewAddressResponse>, Status> {
        let account_id = self.state.ensure_selected_account().await?;
        let response = self
            .state
            .wallet
            .clone()
            .accounts_create_new_address_call(AccountsCreateNewAddressRequest {
                account_id,
                kind: NewAddressKind::Receive,
            })
            .await
            .map_err(status_from_error)?;

        Ok(Response::new(NewAddressResponse { address: response.address.to_string() }))
    }

    async fn shutdown(&self, _request: Request<ShutdownRequest>) -> Result<Response<ShutdownResponse>, Status> {
        if let Some(tx) = self.state.shutdown_tx.lock().await.take() {
            let _ = tx.send(());
        }
        Ok(Response::new(ShutdownResponse {}))
    }

    async fn broadcast(&self, request: Request<BroadcastRequest>) -> Result<Response<BroadcastResponse>, Status> {
        let request = request.into_inner();
        let account_id = self.state.ensure_selected_account().await?;
        let mut tx_ids = Vec::new();

        for transaction in request.transactions {
            let pskb = String::from_utf8(transaction).map_err(|e| Status::invalid_argument(e.to_string()))?;
            let response = self
                .state
                .wallet
                .clone()
                .accounts_pskb_broadcast_call(AccountsPskbBroadcastRequest { account_id, pskb })
                .await
                .map_err(status_from_error)?;
            tx_ids.extend(response.transaction_ids.into_iter().map(|id| id.to_string()));
        }

        Ok(Response::new(BroadcastResponse { tx_i_ds: tx_ids }))
    }

    async fn send(&self, request: Request<SendRequest>) -> Result<Response<SendResponse>, Status> {
        let request = request.into_inner();
        if request.is_send_all {
            return Err(Status::unimplemented("isSendAll is not supported"));
        }

        let wallet_secret = self.state.wallet_secret_from_request(&request.password)?;
        let account_id = self.state.resolve_account_from_addresses(&request.from).await?;
        let destination_address = Address::try_from(request.to_address.as_str()).map_err(status_from_error)?;
        let (fee_rate, priority_fee_sompi) = resolve_fee_policy(request.fee_policy.as_ref());

        let unsigned = self
            .state
            .wallet
            .clone()
            .accounts_pskb_create_call(AccountsPskbCreateRequest {
                account_id,
                wallet_secret: self.state.wallet_secret.clone(),
                payment_secret: self.state.payment_secret.clone(),
                destination: PaymentDestination::from(PaymentOutput::new(destination_address, request.amount)),
                fee_rate,
                priority_fee_sompi,
                payload: None,
            })
            .await
            .map_err(status_from_error)?;

        let signed = self
            .state
            .wallet
            .clone()
            .accounts_pskb_sign_call(AccountsPskbSignRequest {
                account_id,
                pskb: unsigned.pskb,
                wallet_secret,
                payment_secret: self.state.payment_secret.clone(),
                sign_for_address: None,
            })
            .await
            .map_err(status_from_error)?;

        let broadcast = self
            .state
            .wallet
            .clone()
            .accounts_pskb_broadcast_call(AccountsPskbBroadcastRequest {
                account_id,
                pskb: signed.pskb.clone(),
            })
            .await
            .map_err(status_from_error)?;

        Ok(Response::new(SendResponse {
            tx_i_ds: broadcast.transaction_ids.into_iter().map(|id| id.to_string()).collect(),
            signed_transactions: vec![signed.pskb.into_bytes()],
        }))
    }

    async fn sign(&self, request: Request<SignRequest>) -> Result<Response<SignResponse>, Status> {
        let request = request.into_inner();
        let wallet_secret = self.state.wallet_secret_from_request(&request.password)?;
        let account_id = self.state.ensure_selected_account().await?;
        let mut signed_transactions = Vec::new();

        for unsigned in request.unsigned_transactions {
            let pskb = String::from_utf8(unsigned).map_err(|e| Status::invalid_argument(e.to_string()))?;
            let response = self
                .state
                .wallet
                .clone()
                .accounts_pskb_sign_call(AccountsPskbSignRequest {
                    account_id,
                    pskb,
                    wallet_secret: wallet_secret.clone(),
                    payment_secret: self.state.payment_secret.clone(),
                    sign_for_address: None,
                })
                .await
                .map_err(status_from_error)?;
            signed_transactions.push(response.pskb.into_bytes());
        }

        Ok(Response::new(SignResponse { signed_transactions }))
    }

    async fn get_version(&self, _request: Request<GetVersionRequest>) -> Result<Response<GetVersionResponse>, Status> {
        Ok(Response::new(GetVersionResponse { version: velkar_wallet_core::version() }))
    }

    async fn bump_fee(&self, _request: Request<BumpFeeRequest>) -> Result<Response<BumpFeeResponse>, Status> {
        Err(Status::unimplemented("bump fee is not implemented"))
    }
}

fn resolve_fee_policy(policy: Option<&FeePolicy>) -> (Option<f64>, Fees) {
    let Some(policy) = policy else {
        return (None, Fees::SenderPays(0));
    };

    match policy.fee_policy.as_ref() {
        Some(ProtoFeePolicy::MaxFeeRate(rate)) => (Some(*rate), Fees::SenderPays(0)),
        Some(ProtoFeePolicy::ExactFeeRate(rate)) => (Some(*rate), Fees::SenderPays(0)),
        Some(ProtoFeePolicy::MaxFee(fee)) => (None, Fees::SenderPays(*fee)),
        None => (None, Fees::SenderPays(0)),
    }
}

fn status_from_error<E: std::fmt::Display>(err: E) -> Status {
    Status::internal(err.to_string())
}

fn normalize_wrpc_url(url: &str) -> String {
    if url.starts_with("ws://") || url.starts_with("wss://") || url.starts_with("wrpc://") || url.starts_with("wrpcs://") {
        url.to_string()
    } else {
        format!("ws://{url}")
    }
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    velkar_core::log::init_logger(None, "info");

    let network_id = NetworkId::from_str(&args.network).unwrap_or_else(|_| NetworkId::new(NetworkType::Mainnet));
    let wallet = Arc::new(Wallet::try_new(Wallet::local_store()?, Some(Resolver::default()), Some(network_id))?);
    let wallet_secret = Secret::from(args.password);
    let payment_secret = args.payment_secret.map(Secret::from);

    wallet
        .clone()
        .wallet_open_call(WalletOpenRequest {
            wallet_secret: wallet_secret.clone(),
            filename: Some(args.wallet),
            account_descriptors: true,
            legacy_accounts: None,
        })
        .await?;

    wallet
        .clone()
        .connect_call(
            ConnectRequest::default()
                .with_network_id(&network_id)
                .with_url(Some(normalize_wrpc_url(&args.wrpc)))
                .with_require_sync(!args.allow_unsynced),
        )
        .await?;

    wallet.clone().accounts_activate_call(AccountsActivateRequest { account_ids: None }).await?;

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let state = AppState {
        wallet,
        wallet_secret,
        payment_secret,
        account_name: args.account_name,
        shutdown_tx: Arc::new(Mutex::new(Some(shutdown_tx))),
    };

    state.ensure_selected_account().await?;

    let service = WalletdService { state };
    let addr = args.listen.parse()?;

    Server::builder()
        .add_service(VelkarwalletdServer::new(service))
        .serve_with_shutdown(addr, async move {
            let _ = shutdown_rx.await;
        })
        .await?;

    Ok(())
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("failed to build runtime");
    if let Err(err) = runtime.block_on(async_main()) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
