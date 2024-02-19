use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};

use neutron_sdk::{
    bindings::query::{QueryInterchainAccountAddressResponse, QueryRegisteredQueryResponse},
    interchain_queries::v045::queries::{
        BalanceResponse, DelegatorDelegationsResponse, ValidatorResponse,
    },
};

use crate::state::{EraSnapshot, IcaInfo, PoolInfo, QueryKind, Stack, UnstakeInfo};

#[cw_serde]
pub struct InstantiateMsg {
    pub lsd_token_code_id: u64,
    pub stack_fee_receiver: Addr,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryRegisteredQueryResponse)]
    GetRegisteredQuery { query_id: u64 },
    #[returns(QueryRegisteredQueryResponse)]
    GetIcaRegisteredQuery {
        ica_addr: String,
        query_kind: QueryKind,
    },
    #[returns(BalanceResponse)]
    Balance { ica_addr: String },
    #[returns(DelegatorDelegationsResponse)]
    Delegations { pool_addr: String },
    #[returns(ValidatorResponse)]
    Validators { pool_addr: String },
    #[returns(PoolInfo)]
    PoolInfo { pool_addr: String },
    #[returns(Stack)]
    StackInfo {},
    #[returns(EraSnapshot)]
    EraSnapshot { pool_addr: String },
    /// this query goes to neutron and get stored ICA with a specific query
    #[returns(QueryInterchainAccountAddressResponse)]
    InterchainAccountAddress {
        interchain_account_id: String,
        connection_id: String,
    },
    // this query returns ICA from contract store, which saved from acknowledgement
    #[returns((IcaInfo, IcaInfo, Addr))]
    InterchainAccountAddressFromContract { interchain_account_id: String },
    #[returns([UnstakeInfo])]
    UserUnstake {
        pool_addr: String,
        user_neutron_addr: Addr,
    },
    #[returns(String)]
    UserUnstakeIndex {
        pool_addr: String,
        user_neutron_addr: Addr,
    },
    #[returns(Uint128)]
    EraRate { pool_addr: String, era: u64 },
    #[returns(u64)]
    UnbondingSeconds { remote_denom: String },
}

#[cw_serde]
pub struct InitPoolParams {
    pub interchain_account_id: String,
    pub ibc_denom: String,
    pub channel_id_of_ibc_denom: String,
    pub remote_denom: String,
    pub validator_addrs: Vec<String>,
    pub platform_fee_receiver: String,
    pub lsd_code_id: Option<u64>,
    pub lsd_token_name: String,
    pub lsd_token_symbol: String,
    pub minimal_stake: Uint128,
    pub platform_fee_commission: Option<Uint128>,
}

#[cw_serde]
pub struct ConfigStackParams {
    pub stack_fee_receiver: Option<Addr>,
    pub new_admin: Option<Addr>,
    pub stack_fee_commission: Option<Uint128>,
    pub lsd_token_code_id: Option<u64>,
    pub add_entrusted_pool: Option<String>,
    pub remove_entrusted_pool: Option<String>,
}

#[cw_serde]
pub struct ConfigPoolParams {
    pub pool_addr: String,
    pub platform_fee_receiver: Option<String>,
    pub minimal_stake: Option<Uint128>,
    pub unstake_times_limit: Option<u64>,
    pub unbond_commission: Option<Uint128>,
    pub platform_fee_commission: Option<Uint128>,
    pub era_seconds: Option<u64>,
    pub paused: Option<bool>,
    pub lsm_support: Option<bool>,
    pub lsm_pending_limit: Option<u64>,
    pub rate_change_limit: Option<Uint128>,
    pub new_admin: Option<Addr>,
}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterPool {
        connection_id: String,
        interchain_account_id: String,
    },
    InitPool(Box<InitPoolParams>),
    ConfigPool(Box<ConfigPoolParams>),
    ConfigStack(Box<ConfigStackParams>),
    ConfigUnbondingSeconds {
        remote_denom: String,
        unbonding_seconds: Option<u64>,
    },
    OpenChannel {
        pool_addr: String,
        closed_channel_id: String,
    },
    RedeemTokenForShare {
        pool_addr: String,
        tokens: Vec<Coin>,
    },
    Stake {
        neutron_address: String,
        pool_addr: String,
    },
    Unstake {
        amount: Uint128,
        pool_addr: String,
    },
    Withdraw {
        pool_addr: String,
        receiver: Addr,
        unstake_index_list: Vec<u64>,
    },
    PoolRmValidator {
        pool_addr: String,
        validator_addr: String,
    },
    PoolAddValidator {
        pool_addr: String,
        validator_addr: String,
    },
    PoolUpdateValidator {
        pool_addr: String,
        old_validator: String,
        new_validator: String,
    },
    PoolUpdateValidatorsIcq {
        pool_addr: String,
    },
    EraUpdate {
        pool_addr: String,
    },
    EraStake {
        pool_addr: String,
    },
    EraCollectWithdraw {
        pool_addr: String,
    },
    EraRestake {
        pool_addr: String,
    },
    EraActive {
        pool_addr: String,
    },
    StakeLsm {
        neutron_address: String,
        pool_addr: String,
    },
    UpdateIcqUpdatePeriod {
        pool_addr: String,
        new_update_period: u64,
    },
}

#[cw_serde]
pub struct MigrateMsg {}
