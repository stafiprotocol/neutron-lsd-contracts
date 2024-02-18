use crate::helper::{
    self, deal_pool, min_ntrn_ibc_fee, query_icq_register_fee, set_withdraw_sub_msg,
    total_icq_register_fee, CAL_BASE, DEFAULT_ERA_SECONDS,
};
use crate::msg::InitPoolParams;
use crate::state::POOLS;
use crate::state::{ValidatorUpdateStatus, UNBONDING_SECONDS};
use crate::state::{INFO_OF_ICA_ID, STACK};
use crate::{error_conversion::ContractError, state::EraStatus};
use cosmwasm_std::{Addr, Uint128};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use neutron_sdk::query::min_ibc_fee::query_min_ibc_fee;
use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};
use std::ops::{Add, Div, Mul};
use std::{env, vec};

// add execute to config the validator addrs and withdraw address on reply
pub fn execute_init_pool(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    param: InitPoolParams,
) -> NeutronResult<Response<NeutronMsg>> {
    let (pool_ica_info, withdraw_ica_info, _) =
        INFO_OF_ICA_ID.load(deps.storage, param.interchain_account_id.clone())?;

    if param.validator_addrs.is_empty()
        || param.validator_addrs.len() > helper::VALIDATORS_LEN_LIMIT
    {
        return Err(ContractError::ValidatorAddressesListSize {}.into());
    }

    if info.funds.len() != 1 || info.funds[0].denom != helper::FEE_DENOM {
        return Err(ContractError::ParamsErrorFundsNotMatch {}.into());
    }

    let mut pool_info = POOLS.load(deps.storage, pool_ica_info.ica_addr.clone())?;
    pool_info.authorize(&info.sender)?;

    let ibc_fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);
    let total_ibc_fee = helper::total_ibc_fee(ibc_fee.clone());

    if pool_info.status == EraStatus::InitFailed {
        if info.funds[0].amount < total_ibc_fee {
            return Err(ContractError::ParamsErrorFundsNotMatch {}.into());
        }

        return Ok(Response::new().add_submessage(set_withdraw_sub_msg(
            deps,
            pool_info,
            pool_ica_info,
            withdraw_ica_info,
            ibc_fee,
        )?));
    }

    if pool_info.status != EraStatus::RegisterEnded {
        return Err(ContractError::StatusNotAllow {}.into());
    }

    let icq_register_fee = query_icq_register_fee(deps.as_ref())?;
    if info.funds[0].amount
        < total_icq_register_fee(icq_register_fee)
            .mul(Uint128::new(4))
            .add(total_ibc_fee)
    {
        return Err(ContractError::ParamsErrorFundsNotMatch {}.into());
    }

    pool_info.ibc_denom = param.ibc_denom;
    pool_info.channel_id_of_ibc_denom = param.channel_id_of_ibc_denom;
    pool_info.remote_denom = param.remote_denom;
    pool_info.validator_addrs = param.validator_addrs.clone();
    pool_info.platform_fee_receiver = Addr::unchecked(param.platform_fee_receiver);
    pool_info.minimal_stake = param.minimal_stake;

    // option
    if let Some(platform_fee_commission) = param.platform_fee_commission {
        pool_info.platform_fee_commission = platform_fee_commission;
    } else {
        pool_info.platform_fee_commission = Uint128::new(100_000);
    }

    // default
    pool_info.era = 0;
    pool_info.bond = Uint128::zero();
    pool_info.unbond = Uint128::zero();
    pool_info.active = Uint128::zero();
    pool_info.rate = CAL_BASE;
    pool_info.share_tokens = vec![];
    pool_info.total_platform_fee = Uint128::zero();
    pool_info.total_lsd_token_amount = Uint128::zero();
    pool_info.next_unstake_index = 0;
    pool_info.unstake_times_limit = 20;
    pool_info.unbond_commission = Uint128::zero();
    pool_info.paused = false;
    pool_info.lsm_support = false;
    pool_info.lsm_pending_limit = 100;
    pool_info.rate_change_limit = Uint128::zero();
    pool_info.validator_update_status = ValidatorUpdateStatus::End;

    if env!("PROFILE") == "debug" {
        pool_info.era_seconds = 20; // for testing purpose
    } else {
        pool_info.era_seconds = DEFAULT_ERA_SECONDS;
    }

    // cal
    let offset = env.block.time.seconds().div(pool_info.era_seconds);
    pool_info.offset = 0 - (offset as i64);

    let unbonding_seconds = UNBONDING_SECONDS.load(deps.storage, pool_info.remote_denom.clone())?;
    pool_info.unbonding_period = (unbonding_seconds as f64)
        .div(pool_info.era_seconds as f64)
        .ceil() as u64
        + 1;

    let code_id = match param.lsd_code_id {
        Some(lsd_code_id) => lsd_code_id,
        None => STACK.load(deps.storage)?.lsd_token_code_id,
    };

    deal_pool(
        deps,
        env,
        info,
        pool_info,
        pool_ica_info,
        withdraw_ica_info,
        code_id,
        param.lsd_token_name,
        param.lsd_token_symbol,
        ibc_fee,
    )
}
