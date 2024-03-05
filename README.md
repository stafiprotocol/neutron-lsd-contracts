# Neutron LSD Contracts

StaFi Cosmos LSD Stack powered by StaFi Protocol is a suite of software which helps developers deploying LSD project instantly. Thanks to Neutron, we are able to implement liquid staking in smart contract. The key component of it is StakeManager contract, which is responsible for handling staking logic, validator set management, reward distribution, and withdrawals. In addition, LSD token is an cw20 compatible contract, users get LST after stake and it will be burnt after unstake.

## Contracts

| Contract | Version | Description |
| --- | --- |--- |
| Liquid Staking Manager | v0.1.0 | [Code & Documentaion](./contracts/stake_manager/) |
| LSD Token | v0.1.0 | lsd token([Code](./contracts/lsd_token/), [cw20_base](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base)) |

## Build

```sh
make compile
```

## Integration test

```sh
make compile_debug
./scripts/test.sh
```
