# neutron-lsd-contracts

neutron lsd contracts by StaFi Protocol.

## Contracts

| Contract | Version | Description |
| --- | --- |--- |
| stake-manager | v0.1.0 | liquid staking manager |
| lsd_token | v0.1.0 | lsd token([cw20_base](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base)) |

Stake-manager contract holds all LSD functionalities, it consists with many pools, each pool represents a individual project party associated with an admin account who is privileged to config pool's parameters.

### Project party
- `register_pool`: Create pool ICA and withdraw ICA, and bind interchain routes e.g. channel, port
- `init_pool`: Pool should be initiated with validator set, fee reciver, commission rate and lsd token info
- `config_pool`: Update pool configs such as lsm_support, era_seconds, commission fee, fee reciver etc.
- `add_pool_validators`: Adds validators to the pool
- `rm_pool_validator`: Removes validator from the pool.
- `pool_update_validator`: Updates validator information for the pool.

### User
- `stake`:
  - Attached with wasm invocation, users can stake token and get LSD token from source chain by ibc transfer function 
  - Users can call smart contract directly in neutron chain to stake
- `stake_lsm`: Users can stake their LSM to get LSD token avoiding 21 days unboding period
- `unstake`: Anyone who owns LSD token can call this function, LSD token will be burnt and users have to wait unboding period of time to withdraw their assets
- `withdraw`: When unstake become mature, users can withdraw

### Stack

StaFi Team or DAO can config stack parameters:
- default LSD token code id
- administrator address of stack
- entrusted pools: It is a great feature for project party who can rapidly run a LSD token without running its own relay service, StaFi Team will run it instead. It is fully secure as all functions a relay needs to execute are permissionless.

### Token Redemption 

`redeem_token_for_share`: This is a permissionless method that is called in real-time via relay to redeem stake_lsm's LST back to the original chain in exchange for corresponding shares.

### New Era Process

- **Characteristics**: The new era process is permissionless, showcasing the decentralized nature of the Cosmos LSD Stack, allowing anyone to trigger the beginning of a new era. Each step in the process includes sufficient condition checks to prevent the contract from re-processing transactions or prematurely moving to subsequent steps.
- **Conditions**: The new era process can be triggered when a pool meets the conditions for starting a new era (i.e., reaching the time to start the next era).
- **Processes and Functions**:
    - `era_update`: Transfers an era's stored tokens on the neutron chain to an account on the original chain through ICA and interchain transactions.
    - `era_stake`: Handles staking, unstaking, and withdrawal transactions on the original chain.
    - `era_withdraw_collect`: Collects rewards from the previous era into the pool ICA account in preparation for restake.
    - `era_restake`: Restake rewards generated in the previous era.
    - `era_active`: Handles the data changes caused by new stakes or unstakes in the new era process, calculates the new era's rate, and initiates the new era.
- **ICQ Query Frequency Adjustment**: During the new era process, the contract will flexibly update the frequency of ICQ queries as needed to reduce the cost for ICQ relayers.
- When a Redelegate action occurs, `pool_update_validators_icq` must be executed to synchronize the contract content's ICQ with the latest validator-related queries.


## Build

```sh
make compile
```

## Integration test

```sh
make compile_debug
./scripts/test.sh
```
