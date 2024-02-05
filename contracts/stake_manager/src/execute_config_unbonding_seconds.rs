use crate::state::{STACK, UNBONDING_SECONDS};
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

pub fn execute_config_unbonding_seconds(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    remote_denom: String,
    unbonding_seconds: Option<u64>,
) -> NeutronResult<Response<NeutronMsg>> {
    let stack = STACK.load(deps.storage)?;
    stack.authorize(&info.sender)?;

    if let Some(unbonding_seconds) = unbonding_seconds {
        UNBONDING_SECONDS.save(deps.storage, remote_denom, &unbonding_seconds)?;
    } else {
        UNBONDING_SECONDS.remove(deps.storage, remote_denom);
    }

    Ok(Response::default())
}
