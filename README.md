# Velkar Source Snapshot

This repository contains the source code used for the updated Velkar wallet, daemon and miner.

It is meant to be shared with builders, testers and contributors without exposing local binaries, exchange data or other sensitive files.

## What is included

- Velkar daemon source
- Velkar wallet source
- Velkar CPU miner source
- Shared consensus, RPC, mining and utility crates needed to build the tools

## What is not included

- Built binaries
- `target` folders
- Local node databases
- Exchange files
- Wallet secrets
- Logs or temporary files

## Build

### Windows

```powershell
cargo build --release -p kaspad -p velkar-wallet -p velkar-walletd
cargo build --release -p velkar-cpuminer
```

### Linux / Ubuntu

```bash
cargo build --release -p kaspad -p velkar-wallet -p velkar-walletd
cargo build --release -p velkar-cpuminer
```

## Run Velkar daemon

Example for a fresh local bootstrap:

```powershell
velkard.exe --velkarnet --utxoindex --yes --reset-db  --disable-upnp  --listen=0.0.0.0:26111 --rpclisten=0.0.0.0:26110 --rpclisten-borsh=0.0.0.0:17110
```

If you want to connect to an existing Velkar peer, use:

```powershell
velkard.exe --velkarnet --utxoindex --yes --connect=IP_DEL_PEER:26111 --listen=0.0.0.0:26111 --disable-upnp
```

## Run the wallet

```powershell
velkar-wallet.exe
```

Then in the wallet:

```text
network mainnet
connect 127.0.0.1:17110
```

## Run the CPU miner

Solo mining against a local node:

```powershell
velkar-cpuminer.exe --velkard-address 127.0.0.1 --port 26110 --mining-address TU_DIRECCION_VELKAR 
```

Stratum mining example:

```powershell
velkar-cpuminer.exe --velkard-address stratum+tcp://pool:port --mining-address wallet (pool no active now)
```

## Notes

- This codebase is still based on the Velkar/Kaspa Rust architecture, so some internal crate names may still reference upstream names.
- For GitHub publishing, this snapshot is the clean source tree only.
- For releases, build artifacts should be generated separately from this source folder.
