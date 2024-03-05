use crate::state::{DECIMALS, STACK};
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

pub fn execute_config_decimals(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    remote_denom: String,
    decimals: Option<u8>,
) -> NeutronResult<Response<NeutronMsg>> {
    let stack = STACK.load(deps.storage)?;
    stack.authorize(&info.sender)?;

    if let Some(decimals) = decimals {
        DECIMALS.save(deps.storage, remote_denom, &decimals)?;
    } else {
        DECIMALS.remove(deps.storage, remote_denom);
    }

    Ok(Response::default())
}
