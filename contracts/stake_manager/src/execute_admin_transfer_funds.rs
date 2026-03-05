use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint128};

use crate::error_conversion::ContractError;
use crate::helper::{self, gen_msg_send, DEFAULT_TIMEOUT_SECONDS};
use crate::state::{SudoPayload, TxType, INFO_OF_ICA_ID, POOLS};
use crate::tx_callback::msg_with_sudo_callback;
use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

pub fn execute_admin_transfer_funds(
    mut deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    pool_addr: String,
    receiver: String,
    amount: Uint128,
) -> NeutronResult<Response<NeutronMsg>> {
    let pool_info = POOLS.load(deps.storage, pool_addr.clone())?;

    pool_info.authorize(&info.sender)?;

    if amount.is_zero() {
        return Err(ContractError::EncodeErrZeroWithdrawAmount {}.into());
    }

    let ibc_fee = helper::check_ibc_fee(deps.as_ref(), &info)?;

    let (pool_ica_info, _, _) = INFO_OF_ICA_ID.load(deps.storage, pool_info.ica_id.clone())?;

    let send_msg = gen_msg_send(
        pool_addr.clone(),
        receiver.clone(),
        pool_info.remote_denom.clone(),
        amount.to_string(),
    )?;

    let cosmos_msg = NeutronMsg::submit_tx(
        pool_ica_info.ctrl_connection_id,
        pool_info.ica_id.clone(),
        vec![send_msg],
        "".to_string(),
        DEFAULT_TIMEOUT_SECONDS,
        ibc_fee,
    );

    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            port_id: pool_ica_info.ctrl_port_id,
            message: format!("{}_{}", amount, receiver),
            pool_addr: pool_addr.clone(),
            tx_type: TxType::AdminTransfer,
        },
    )?;

    Ok(Response::default().add_submessage(submsg))
}

pub fn sudo_admin_transfer_callback(
    payload: SudoPayload,
) -> NeutronResult<Response<NeutronMsg>> {
    Ok(Response::new()
        .add_attribute("action", "admin_transfer_callback")
        .add_attribute("pool_addr", payload.pool_addr)
        .add_attribute("transfer", payload.message))
}

pub fn sudo_admin_transfer_failed_callback(
    payload: SudoPayload,
) -> NeutronResult<Response<NeutronMsg>> {
    Ok(Response::new()
        .add_attribute("action", "admin_transfer_failed_callback")
        .add_attribute("pool_addr", payload.pool_addr)
        .add_attribute("transfer", payload.message))
}
