# solana-orca-whirlpool-rebalancer

[![Solana](https://img.shields.io/badge/Solana-Anchor%200.30-14F195?logo=solana)](https://solana.com)
[![Rust](https://img.shields.io/badge/Rust-1.79+-orange.svg?logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Concentrated liquidity Whirlpool range adjuster and tick spacing liquidity manager on Solana.

## Architecture

```mermaid
graph TD
    Client([Client SDK]) -->|CPI Instruction| Program[Anchor Program Entrypoint]
    Program -->|Validate Seeds & Bump| PDA[Program Derived Address]
    Program -->|State Mutex| StateAccount[On-Chain State Account]
```

## Highlights
- **Anchor 0.30 Framework**: Clean type-safe account validation macros.
- **Zero-Copy Deserialization**: High throughput processing under extreme network load.
- **SPL Token Compatibility**: Native support for standard SPL Token and Token-2022 extensions.

## Build & Test
```bash
anchor build
anchor test
```

## License
MIT
