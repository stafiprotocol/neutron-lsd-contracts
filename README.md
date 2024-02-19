# neutron-lsd-contracts

neutron lsd contracts by StaFi Protocol.

## Contracts

| Contract | Version | Description |
| --- | --- |--- |
| stake-manager | v0.1.0 | liquid staking manager [Code & Documentaion](./contracts/stake_manager/) |
| lsd_token | v0.1.0 | lsd token([cw20_base](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base)) |

## Build

```sh
make compile
```

## Integration test

```sh
make compile_debug
./scripts/test.sh
```
