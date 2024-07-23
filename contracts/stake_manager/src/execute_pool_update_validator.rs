use crate::error_conversion::ContractError;
use crate::helper::{self};
use crate::state::INFO_OF_ICA_ID;
use crate::state::{ValidatorUpdateStatus, POOLS};
use crate::{
    helper::gen_redelegate_txs,
    state::{SudoPayload, TxType},
    tx_callback::msg_with_sudo_callback,
};
use crate::{helper::DEFAULT_TIMEOUT_SECONDS, query::query_delegation_by_addr};
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};
use std::vec;

pub fn execute_pool_update_validator(
    mut deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    pool_addr: String,
    old_validator: String,
    new_validator: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut pool_info: crate::state::PoolInfo = POOLS.load(deps.storage, pool_addr.clone())?;
    pool_info.authorize(&info.sender)?;
    pool_info.require_era_ended()?;
    pool_info.require_update_validator_ended()?;

    if !pool_info.validator_addrs.contains(&old_validator) {
        return Err(ContractError::OldValidatorNotExist {}.into());
    }
    if pool_info.validator_addrs.contains(&new_validator) {
        return Err(ContractError::NewValidatorAlreadyExist {}.into());
    }

    let delegations = query_delegation_by_addr(
        deps.as_ref(),
        pool_addr.clone(),
        pool_info.sdk_greater_or_equal_v047,
    )?;

    let mut new_validators = pool_info.validator_addrs.clone();
    new_validators.retain(|x| x.as_str() != old_validator);
    new_validators.push(new_validator.clone());

    let mut msgs = vec![];

    for delegation in delegations.delegations {
        if delegation.validator != old_validator {
            continue;
        }
        let stake_amount = delegation.amount.amount;

        if stake_amount.is_zero() {
            break;
        }

        let any_msg = gen_redelegate_txs(
            pool_addr.clone(),
            delegation.validator.clone(),
            new_validator.clone(),
            pool_info.remote_denom.clone(),
            stake_amount,
        );

        msgs.push(any_msg);
    }
    let (pool_ica_info, _, _) = INFO_OF_ICA_ID.load(deps.storage, pool_info.ica_id.clone())?;

    // let remove_msg_old_query = NeutronMsg::remove_interchain_query(registere_query_id);
    let mut resp = Response::default(); // .add_message(remove_msg_old_query)

    if !msgs.is_empty() {
        let ibc_fee = helper::check_ibc_fee(deps.as_ref(), &info)?;

        let submsg_redelegate = msg_with_sudo_callback(
            deps.branch(),
            NeutronMsg::submit_tx(
                pool_ica_info.ctrl_connection_id.clone(),
                pool_info.ica_id.clone(),
                msgs,
                "".to_string(),
                DEFAULT_TIMEOUT_SECONDS,
                ibc_fee,
            ),
            SudoPayload {
                port_id: pool_ica_info.ctrl_port_id,
                pool_addr: pool_ica_info.ica_addr.clone(),
                message: new_validators
                    .clone()
                    .into_iter()
                    .map(|index| index.to_string())
                    .collect::<Vec<String>>()
                    .join("_"),
                tx_type: TxType::UpdateValidator,
            },
        )?;

        pool_info.validator_update_status = ValidatorUpdateStatus::Start;

        resp = resp.add_submessage(submsg_redelegate)
    } else {
        pool_info.validator_update_status = ValidatorUpdateStatus::WaitQueryUpdate;
    }

    pool_info.validator_addrs = new_validators;
    POOLS.save(deps.storage, pool_addr.clone(), &pool_info)?;

    Ok(resp)
}

pub fn sudo_update_validator_callback(
    deps: DepsMut,
    payload: SudoPayload,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut pool_info = POOLS.load(deps.storage, payload.pool_addr.clone())?;

    let new_validators: Vec<String> = payload.message.split('_').map(String::from).collect();

    pool_info.validator_addrs = new_validators;
    pool_info.validator_update_status = ValidatorUpdateStatus::WaitQueryUpdate;

    POOLS.save(deps.storage, payload.pool_addr.clone(), &pool_info)?;

    Ok(Response::new())
}

pub fn sudo_update_validator_failed_callback(
    deps: DepsMut,
    payload: SudoPayload,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut pool_info = POOLS.load(deps.storage, payload.pool_addr.clone())?;

    pool_info.validator_update_status = ValidatorUpdateStatus::End;

    POOLS.save(deps.storage, payload.pool_addr, &pool_info)?;

    Ok(Response::new())
}
