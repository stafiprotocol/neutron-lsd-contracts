use cosmwasm_std::{DepsMut, MessageInfo, Response};

use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

use crate::state::STACK;
use crate::{msg::ConfigPoolStackFeeParams, state::POOLS};

pub fn execute_config_pool_stack_fee(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    param: ConfigPoolStackFeeParams,
) -> NeutronResult<Response<NeutronMsg>> {
    let stack = STACK.load(deps.storage)?;
    stack.authorize(&info.sender)?;

    let mut pool_info = POOLS.load(deps.storage, param.pool_addr.clone())?;
    pool_info.stack_fee_commission = param.stack_fee_commission;

    POOLS.save(deps.storage, param.pool_addr.clone(), &pool_info)?;

    Ok(Response::default())
}
