use crate::helper::get_update_pool_icq_msgs;
use crate::helper::DEFAULT_UPDATE_PERIOD;
use crate::state::POOLS;
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::{bindings::msg::NeutronMsg, NeutronResult};

pub fn update_icq_update_period(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    pool_addr: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let pool_info = POOLS.load(deps.storage, pool_addr.clone())?;
    pool_info.authorize(&info.sender)?;

    let update_pool_icq_msgs = get_update_pool_icq_msgs(
        deps,
        pool_addr,
        pool_info.ica_id.clone(),
        DEFAULT_UPDATE_PERIOD,
    )?;

    Ok(Response::default().add_messages(update_pool_icq_msgs))
}
