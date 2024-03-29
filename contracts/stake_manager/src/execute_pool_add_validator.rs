use cosmwasm_std::{DepsMut, MessageInfo, Response};

use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

use crate::helper::deal_validators_icq_update;
use crate::state::{INFO_OF_ICA_ID, POOLS};
use crate::{error_conversion::ContractError, helper};

pub fn execute_add_pool_validators(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    pool_addr: String,
    validator_addr: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut pool_info = POOLS.load(deps.storage, pool_addr.clone())?;
    pool_info.authorize(&info.sender)?;
    pool_info.require_era_ended()?;

    if pool_info.validator_addrs.len() >= helper::VALIDATORS_LEN_LIMIT {
        return Err(ContractError::ValidatorAddressesListSize {}.into());
    }
    if pool_info.validator_addrs.contains(&validator_addr) {
        return Err(ContractError::ValidatorAlreadyExit {}.into());
    }
    pool_info.validator_addrs.push(validator_addr);

    POOLS.save(deps.storage, pool_addr.clone(), &pool_info)?;

    let (pool_ica_info, _, _) = INFO_OF_ICA_ID.load(deps.storage, pool_info.ica_id.clone())?;

    deal_validators_icq_update(deps, pool_addr, pool_info, pool_ica_info.ctrl_connection_id)
}
