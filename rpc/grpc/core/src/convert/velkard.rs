use crate::protowire::{VelkardRequest, VelkardResponse, velkard_request};

impl From<velkard_request::Payload> for VelkardRequest {
    fn from(item: velkard_request::Payload) -> Self {
        VelkardRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<VelkardRequest> for VelkardRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<VelkardResponse> for VelkardResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod velkard_request_convert {
    use crate::protowire::*;
    use velkar_rpc_core::{RpcError, RpcResult};

    impl_into_velkard_request!(Shutdown);
    impl_into_velkard_request!(SubmitBlock);
    impl_into_velkard_request!(GetBlockTemplate);
    impl_into_velkard_request!(GetBlock);
    impl_into_velkard_request!(GetInfo);

    impl_into_velkard_request!(GetCurrentNetwork);
    impl_into_velkard_request!(GetPeerAddresses);
    impl_into_velkard_request!(GetSink);
    impl_into_velkard_request!(GetMempoolEntry);
    impl_into_velkard_request!(GetMempoolEntries);
    impl_into_velkard_request!(GetConnectedPeerInfo);
    impl_into_velkard_request!(AddPeer);
    impl_into_velkard_request!(SubmitTransaction);
    impl_into_velkard_request!(SubmitTransactionReplacement);
    impl_into_velkard_request!(GetSubnetwork);
    impl_into_velkard_request!(GetVirtualChainFromBlock);
    impl_into_velkard_request!(GetBlocks);
    impl_into_velkard_request!(GetBlockCount);
    impl_into_velkard_request!(GetBlockDagInfo);
    impl_into_velkard_request!(ResolveFinalityConflict);
    impl_into_velkard_request!(GetHeaders);
    impl_into_velkard_request!(GetUtxosByAddresses);
    impl_into_velkard_request!(GetBalanceByAddress);
    impl_into_velkard_request!(GetBalancesByAddresses);
    impl_into_velkard_request!(GetSinkBlueScore);
    impl_into_velkard_request!(Ban);
    impl_into_velkard_request!(Unban);
    impl_into_velkard_request!(EstimateNetworkHashesPerSecond);
    impl_into_velkard_request!(GetMempoolEntriesByAddresses);
    impl_into_velkard_request!(GetCoinSupply);
    impl_into_velkard_request!(Ping);
    impl_into_velkard_request!(GetMetrics);
    impl_into_velkard_request!(GetConnections);
    impl_into_velkard_request!(GetSystemInfo);
    impl_into_velkard_request!(GetServerInfo);
    impl_into_velkard_request!(GetSyncStatus);
    impl_into_velkard_request!(GetDaaScoreTimestampEstimate);
    impl_into_velkard_request!(GetFeeEstimate);
    impl_into_velkard_request!(GetFeeEstimateExperimental);
    impl_into_velkard_request!(GetCurrentBlockColor);
    impl_into_velkard_request!(GetUtxoReturnAddress);
    impl_into_velkard_request!(GetVirtualChainFromBlockV2);

    impl_into_velkard_request!(NotifyBlockAdded);
    impl_into_velkard_request!(NotifyNewBlockTemplate);
    impl_into_velkard_request!(NotifyUtxosChanged);
    impl_into_velkard_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_velkard_request!(NotifyFinalityConflict);
    impl_into_velkard_request!(NotifyVirtualDaaScoreChanged);
    impl_into_velkard_request!(NotifyVirtualChainChanged);
    impl_into_velkard_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_velkard_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_velkard_request_ex!(velkar_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_velkard_request;

    macro_rules! impl_into_velkard_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for velkard_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for VelkardRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for velkard_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for VelkardRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&velkard_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &velkard_request::Payload) -> RpcResult<Self> {
                    if let velkard_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&VelkardRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &VelkardRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("VelkarRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for VelkardRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(velkard_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for velkard_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    velkard_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_velkard_request_ex;
}

pub mod velkard_response_convert {
    use crate::protowire::*;
    use velkar_rpc_core::{RpcError, RpcResult};

    impl_into_velkard_response!(Shutdown);
    impl_into_velkard_response!(SubmitBlock);
    impl_into_velkard_response!(GetBlockTemplate);
    impl_into_velkard_response!(GetBlock);
    impl_into_velkard_response!(GetInfo);
    impl_into_velkard_response!(GetCurrentNetwork);

    impl_into_velkard_response!(GetPeerAddresses);
    impl_into_velkard_response!(GetSink);
    impl_into_velkard_response!(GetMempoolEntry);
    impl_into_velkard_response!(GetMempoolEntries);
    impl_into_velkard_response!(GetConnectedPeerInfo);
    impl_into_velkard_response!(AddPeer);
    impl_into_velkard_response!(SubmitTransaction);
    impl_into_velkard_response!(SubmitTransactionReplacement);
    impl_into_velkard_response!(GetSubnetwork);
    impl_into_velkard_response!(GetVirtualChainFromBlock);
    impl_into_velkard_response!(GetBlocks);
    impl_into_velkard_response!(GetBlockCount);
    impl_into_velkard_response!(GetBlockDagInfo);
    impl_into_velkard_response!(ResolveFinalityConflict);
    impl_into_velkard_response!(GetHeaders);
    impl_into_velkard_response!(GetUtxosByAddresses);
    impl_into_velkard_response!(GetBalanceByAddress);
    impl_into_velkard_response!(GetBalancesByAddresses);
    impl_into_velkard_response!(GetSinkBlueScore);
    impl_into_velkard_response!(Ban);
    impl_into_velkard_response!(Unban);
    impl_into_velkard_response!(EstimateNetworkHashesPerSecond);
    impl_into_velkard_response!(GetMempoolEntriesByAddresses);
    impl_into_velkard_response!(GetCoinSupply);
    impl_into_velkard_response!(Ping);
    impl_into_velkard_response!(GetMetrics);
    impl_into_velkard_response!(GetConnections);
    impl_into_velkard_response!(GetSystemInfo);
    impl_into_velkard_response!(GetServerInfo);
    impl_into_velkard_response!(GetSyncStatus);
    impl_into_velkard_response!(GetDaaScoreTimestampEstimate);
    impl_into_velkard_response!(GetFeeEstimate);
    impl_into_velkard_response!(GetFeeEstimateExperimental);
    impl_into_velkard_response!(GetCurrentBlockColor);
    impl_into_velkard_response!(GetUtxoReturnAddress);
    impl_into_velkard_response!(GetVirtualChainFromBlockV2);

    impl_into_velkard_notify_response!(NotifyBlockAdded);
    impl_into_velkard_notify_response!(NotifyNewBlockTemplate);
    impl_into_velkard_notify_response!(NotifyUtxosChanged);
    impl_into_velkard_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_velkard_notify_response!(NotifyFinalityConflict);
    impl_into_velkard_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_velkard_notify_response!(NotifyVirtualChainChanged);
    impl_into_velkard_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_velkard_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_velkard_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_velkard_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_velkard_response_ex!(velkar_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_velkard_response_base!(velkar_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_velkard_response;

    macro_rules! impl_into_velkard_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for velkard_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    velkard_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for VelkardResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(velkard_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_velkard_response_base;

    macro_rules! impl_into_velkard_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for velkard_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    velkard_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for VelkardResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for velkard_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    velkard_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for VelkardResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_velkard_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&velkard_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &velkard_response::Payload) -> RpcResult<Self> {
                    if let velkard_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&VelkardResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &VelkardResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("VelkarResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_velkard_response_ex;

    macro_rules! impl_into_velkard_notify_response {
        ($name:tt) => {
            impl_into_velkard_response!($name);

            paste::paste! {
                impl_into_velkard_notify_response_ex!(velkar_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_velkard_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_velkard_notify_response_ex!(velkar_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_velkard_notify_response;

    macro_rules! impl_into_velkard_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_velkard_notify_response_ex;
}
