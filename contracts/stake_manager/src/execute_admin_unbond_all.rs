use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgUndelegate;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::prost::Message;
use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response};

use crate::error_conversion::ContractError;
use crate::helper::{self, DEFAULT_TIMEOUT_SECONDS};
use crate::query::query_delegation_by_addr;
use crate::state::{SudoPayload, TxType, INFO_OF_ICA_ID, POOLS};
use crate::tx_callback::msg_with_sudo_callback;
use neutron_sdk::bindings::types::ProtobufAny;
use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

pub fn execute_admin_unbond_all(
    mut deps: DepsMut<NeutronQuery>,
    _env: Env,
    info: MessageInfo,
    pool_addr: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut pool_info = POOLS.load(deps.storage, pool_addr.clone())?;

    pool_info.authorize(&info.sender)?;
    pool_info.require_era_ended()?;

    let ibc_fee = helper::check_ibc_fee(deps.as_ref(), &info)?;

    let delegations = query_delegation_by_addr(
        deps.as_ref(),
        pool_addr.clone(),
        pool_info.sdk_greater_or_equal_v047,
    )
    .map_err(|e| ContractError::DelegationsNotExist(e.to_string()))?;

    let active_delegations: Vec<_> = delegations
        .delegations
        .iter()
        .filter(|d| !d.amount.amount.is_zero())
        .collect();

    if active_delegations.is_empty() {
        return Err(ContractError::DelegationsNotExist("no active delegations".to_string()).into());
    }

    let mut msgs = vec![];
    let mut validator_list = vec![];

    for delegation in &active_delegations {
        let undelegate_msg = MsgUndelegate {
            delegator_address: pool_addr.clone(),
            validator_address: delegation.validator.clone(),
            amount: Some(Coin {
                denom: pool_info.remote_denom.clone(),
                amount: delegation.amount.amount.to_string(),
            }),
        };

        let mut buf = Vec::new();
        buf.reserve(undelegate_msg.encoded_len());

        if let Err(e) = undelegate_msg.encode(&mut buf) {
            return Err(ContractError::EncodeError(e.to_string()).into());
        }

        msgs.push(ProtobufAny {
            type_url: "/cosmos.staking.v1beta1.MsgUndelegate".to_string(),
            value: Binary::from(buf),
        });

        validator_list.push(delegation.validator.clone());
    }

    let msg_str = validator_list.join("_");

    pool_info.paused = true;
    POOLS.save(deps.storage, pool_addr.clone(), &pool_info)?;

    let (pool_ica_info, _, _) = INFO_OF_ICA_ID.load(deps.storage, pool_info.ica_id.clone())?;

    let cosmos_msg = NeutronMsg::submit_tx(
        pool_ica_info.ctrl_connection_id,
        pool_info.ica_id.clone(),
        msgs,
        "".to_string(),
        DEFAULT_TIMEOUT_SECONDS,
        ibc_fee,
    );

    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            port_id: pool_ica_info.ctrl_port_id,
            message: msg_str,
            pool_addr: pool_addr.clone(),
            tx_type: TxType::AdminUnbondAll,
        },
    )?;

    Ok(Response::default().add_submessage(submsg))
}

pub fn sudo_admin_unbond_all_callback(
    payload: SudoPayload,
) -> NeutronResult<Response<NeutronMsg>> {
    Ok(Response::new()
        .add_attribute("action", "admin_unbond_all_callback")
        .add_attribute("pool_addr", payload.pool_addr)
        .add_attribute("validators", payload.message))
}

pub fn sudo_admin_unbond_all_failed_callback(
    payload: SudoPayload,
) -> NeutronResult<Response<NeutronMsg>> {
    Ok(Response::new()
        .add_attribute("action", "admin_unbond_all_failed_callback")
        .add_attribute("pool_addr", payload.pool_addr)
        .add_attribute("validators", payload.message))
}
