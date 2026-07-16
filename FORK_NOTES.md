Velkar Fork Notes

This folder is an isolated fork workspace.

Changes made:
- New hybrid PoW algorithm in `consensus/pow/src/lib.rs`.
- Block rate reduced from 10 BPS to 5 BPS in `consensus/core/src/config/params.rs`.
- Network isolation defaults changed:
  - Prefixed network name: `velkar-*` instead of `velkar-*`.
  - Default P2P/RPC ports moved from 16xxx/17xxx/18xxx to 26xxx/27xxx/28xxx.
  - File: `consensus/core/src/network.rs`.
- Address HRP/prefix changed:
  - `velkar` -> `velkar`
  - `velkartest` -> `velkartest`
  - `velkarsim` -> `velkarsim`
  - `velkardev` -> `velkardev`
  - File: `crypto/addresses/src/lib.rs`.

Notes:
- Compile check could not run fully in this environment because cargo failed downloading crates (SSL credentials issue).
- This fork is designed to avoid accidental overlap with standard Velkar network defaults.

