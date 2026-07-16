# Velkar Source Snapshot

This repository contains the source code for the updated Velkar wallet, daemon and CPU miner.

It is intended for public sharing and testing without exposing local binaries, explorer code, exchange data, node databases or other sensitive files.

## Included

- Velkar daemon source
- Velkar wallet source
- Velkar CPU miner source
- Shared consensus, RPC, mining and utility crates needed to build the tools

## Excluded

- Built binaries
- `target` folders
- Explorer helper code
- Local node databases
- Logs and temporary files
- Exchange backups and private data

## Build

### Windows

```powershell
cargo build --release -p velkard -p velkar-wallet -p velkar-walletd

```

### Ubuntu / Linux

```bash
cargo build --release -p velkard -p velkar-wallet -p velkar-walletd

```

## Run the daemon

```powershell
velkard.exe --velkarnet --utxoindex --yes --reset-db --disable-upnp --addpeer=212.227.144.255:26111 --listen=0.0.0.0:26111 --rpclisten=0.0.0.0:26110 --rpclisten-borsh=0.0.0.0:17110
```

## Run the wallet

```powershell
velkar-wallet.exe
```

Then inside the wallet:

```text
network mainnet
connect 127.0.0.1:17110
```

## Run the miner

Solo mining:

```powershell
velkar-cpuminer.exe --velkard-address 127.0.0.1 --port 26110 --mining-address TU_DIRECCION_VELKAR --mine-when-not-synced
```

Stratum mining:

```powershell
velkar-cpuminer.exe --velkard-address stratum+tcp://pool.liquidpool.net:4001 --mining-address adrislipknot --mine-when-not-synced
```
