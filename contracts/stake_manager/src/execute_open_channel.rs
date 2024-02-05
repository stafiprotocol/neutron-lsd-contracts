use std::vec;

use cosmwasm_std::{DepsMut, MessageInfo, Response};

use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

use crate::error_conversion::ContractError;
use crate::{
    helper::get_withdraw_ica_id,
    state::{INFO_OF_ICA_ID, POOLS},
};

pub fn execute_open_channel(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    pool_addr: String,
    closed_channel_id: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let pool_info = POOLS.load(deps.storage, pool_addr)?;
    pool_info.authorize(&info.sender)?;

    let mut msgs = vec![];

    let register_fee = if !info.funds.is_empty() {
        Some(info.funds)
    } else {
        None
    };

    let (pool_ica_info, withdraw_ica_info, _) =
        INFO_OF_ICA_ID.load(deps.storage, pool_info.ica_id.clone())?;
    if closed_channel_id.eq(&pool_ica_info.ctrl_channel_id.clone()) {
        let register_pool_msg = NeutronMsg::register_interchain_account(
            pool_ica_info.ctrl_connection_id.clone(),
            pool_info.ica_id.clone(),
            register_fee,
        );
        msgs.push(register_pool_msg);
    } else if closed_channel_id.eq(&withdraw_ica_info.ctrl_channel_id.clone()) {
        let register_withdraw_msg = NeutronMsg::register_interchain_account(
            withdraw_ica_info.ctrl_connection_id.clone(),
            get_withdraw_ica_id(pool_info.ica_id),
            register_fee,
        );
        msgs.push(register_withdraw_msg);
    } else {
        return Err(ContractError::ClosedChannelIdUnmatch {}.into());
    }

    Ok(Response::default().add_messages(msgs))
}
