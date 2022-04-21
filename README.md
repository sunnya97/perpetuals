# Margined Protocol Perpertuals

[![Continuous Integration](https://github.com/margined-protocol/perpetuals/actions/workflows/ci.yml/badge.svg)](https://github.com/margined-protocol/perpetuals/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/margined-protocol/perpetuals/branch/main/graph/badge.svg?token=FDMKT04UWK)](https://codecov.io/gh/margined-protocol/perpetuals)


This repo contains a the Margined Protocol a decentralized perpetual contract protocol on the Terra Blockchain.

## Contracts

| Contract                                                | Reference | Description                                                                                           |
| ------------------------------------------------------- | --------- | ----------------------------------------------------------------------------------------------------- |
| [`Margin Engine`](./contracts/margined-engine)          | [doc]()   | Margin engine that manages users positions and the collateral management                              |
| [`vAMM`](./contracts/margined-vamm)                     | [doc]()   | Virtual AMM enabling users to take perpetual positions                                                |
| [`Insurance Fund`](./contracts/margined_insurance_fund) | [doc]()   | Contract that holds funds to cover shortfalls                                                         |
| [`Fee Pool`](./contracts/margined-vamm)                 | [doc]()   | Contract that accrues the fees generated by protocol to be redistributed to `$MRG` token holders      |
| [`Price Feed`](./contracts/margined-price-feed)         | [doc]()   | Integration contract for the data oracles and other data related logic                                |
| [`Governance`](./contracts/margined-price-feed)         | [doc]()   | Community governance contract that managers the protocol                                              |
| [`Factory`](./contracts/margined-price-feed)            | [doc]()   | Manages the deployment and upgrade of perpetual contracts                                             |

## Get started

### Environment Setup

- Rust v1.44.1+
- `wasm32-unknown-unknown` target
- Docker

1. Install `rustup` via https://rustup.rs/

2. Run the following:

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Make sure [Docker](https://www.docker.com/) is installed

### Unit / Integration Tests

To run the tests after installing pre-requisites do the following:

```sh
cargo test
```
### Build

Clone this repository and build the source code:
```
git clone git@github.com:margined-protocol/perpetuals.git
cd perpetuals
cargo build
```
