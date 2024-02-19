use cosmwasm_std::{DepsMut, MessageInfo, Response};

use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

use crate::msg::ConfigStackParams;
use crate::state::STACK;

pub fn execute_config_stack(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    param: ConfigStackParams,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut stack = STACK.load(deps.storage)?;
    stack.authorize(&info.sender)?;

    if let Some(stack_fee_receiver) = param.stack_fee_receiver {
        stack.stack_fee_receiver = stack_fee_receiver
    }
    if let Some(stack_fee_commission) = param.stack_fee_commission {
        stack.stack_fee_commission = stack_fee_commission;
    }
    if let Some(new_admin) = param.new_admin {
        stack.admin = new_admin;
    }
    if let Some(lsd_token_code_id) = param.lsd_token_code_id {
        stack.lsd_token_code_id = lsd_token_code_id;
    }
    if let Some(add_entrusted_pool) = param.add_entrusted_pool {
        if !stack.entrusted_pools.contains(&add_entrusted_pool) {
            stack.entrusted_pools.push(add_entrusted_pool);
        }
    }
    if let Some(remove_entrusted_pool) = param.remove_entrusted_pool {
        if stack.entrusted_pools.contains(&remove_entrusted_pool) {
            stack
                .entrusted_pools
                .retain(|p| p.to_string() != remove_entrusted_pool);
        }
    }

    STACK.save(deps.storage, &stack)?;

    Ok(Response::default())
}
