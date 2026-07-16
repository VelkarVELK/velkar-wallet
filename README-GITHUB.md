Velkar GitHub source snapshot

This folder contains the source code needed to build the updated Velkar wallet, daemon and miner.
No binaries, targets, local data, exchange files or other sensitive assets are included.

Build:
  cargo build --release -p kaspad -p velkar-wallet -p velkar-walletd
  cargo build --release -p velkar-cpuminer
