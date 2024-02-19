use std::ops::Div;

use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use neutron_sdk::interchain_txs::helpers::get_port_id;
use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronResult,
};

use crate::error_conversion::ContractError;
use crate::helper;
use crate::{
    helper::{get_withdraw_ica_id, ICA_WITHDRAW_SUFIX, INTERCHAIN_ACCOUNT_ID_LEN_LIMIT},
    state::{IcaInfo, PoolInfo, INFO_OF_ICA_ID, POOLS},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct OpenAckVersion {
    version: String,
    controller_connection_id: String,
    host_connection_id: String,
    address: String,
    encoding: String,
    tx_type: String,
}

pub fn execute_register_pool(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    connection_id: String,
    interchain_account_id: String,
) -> NeutronResult<Response<NeutronMsg>> {
    if interchain_account_id.trim().is_empty()
        || interchain_account_id.contains(".")
        || interchain_account_id.contains("-")
        || interchain_account_id.contains(ICA_WITHDRAW_SUFIX)
        || interchain_account_id.len() > INTERCHAIN_ACCOUNT_ID_LEN_LIMIT
    {
        return Err(ContractError::InvalidInterchainAccountId {}.into());
    }

    if INFO_OF_ICA_ID.has(deps.storage, interchain_account_id.clone()) {
        return Err(ContractError::InterchainAccountIdAlreadyExist {}.into());
    }

    if info.funds.len() != 1 || info.funds[0].denom != helper::FEE_DENOM {
        return Err(ContractError::ParamsErrorFundsNotMatch {}.into());
    }
    let register_fee = Some(vec![Coin::new(
        info.funds[0].amount.u128().div(2),
        info.funds[0].denom.clone(),
    )]);

    let register_pool_msg = NeutronMsg::register_interchain_account(
        connection_id.clone(),
        interchain_account_id.clone(),
        register_fee.clone(),
    );

    let withdraw_ica_id = get_withdraw_ica_id(interchain_account_id.clone());
    let register_withdraw_msg = NeutronMsg::register_interchain_account(
        connection_id.clone(),
        withdraw_ica_id.clone(),
        register_fee,
    );

    let ctrl_port_id_of_pool = get_port_id(
        env.contract.address.as_str(),
        &interchain_account_id.clone(),
    );
    let ctrl_port_id_of_withdraw = get_port_id(env.contract.address.as_str(), &withdraw_ica_id);

    INFO_OF_ICA_ID.save(
        deps.storage,
        interchain_account_id.clone(),
        &(
            IcaInfo {
                ctrl_connection_id: connection_id.clone(),
                ctrl_port_id: ctrl_port_id_of_pool,
                ..Default::default()
            },
            IcaInfo {
                ctrl_connection_id: connection_id.clone(),
                ctrl_port_id: ctrl_port_id_of_withdraw,
                ..Default::default()
            },
            info.sender,
        ),
    )?;

    Ok(Response::default().add_messages(vec![register_pool_msg, register_withdraw_msg]))
}

// handler register pool
pub fn sudo_open_ack(
    deps: DepsMut,
    ctrl_port_id: String,
    ctrl_channel_id: String,
    counterparty_channel_id: String,
    counterparty_version: String,
) -> NeutronResult<Response<NeutronMsg>> {
    // The version variable contains a JSON value with multiple fields,
    // including the generated account address.
    let counterparty_version: OpenAckVersion =
        serde_json_wasm::from_str(counterparty_version.as_str())
            .map_err(|_| ContractError::CantParseCounterpartyVersion {})?;

    let port_id_parts: Vec<String> = ctrl_port_id.split('.').map(String::from).collect();
    if port_id_parts.len() != 2 {
        return Err(ContractError::CounterpartyVersionNotMatch {}.into());
    }

    let ica_id_raw = port_id_parts.get(1).unwrap();
    let mut is_pool = true;
    let ica_id = if ica_id_raw.contains(ICA_WITHDRAW_SUFIX) {
        is_pool = false;
        ica_id_raw
            .strip_suffix(ICA_WITHDRAW_SUFIX)
            .unwrap()
            .to_string()
    } else {
        ica_id_raw.clone()
    };

    let (mut pool_ica_info, mut withdraw_ica_info, admin) =
        INFO_OF_ICA_ID.load(deps.storage, ica_id.clone())?;

    if is_pool {
        pool_ica_info.ctrl_channel_id = ctrl_channel_id;
        pool_ica_info.ctrl_port_id = ctrl_port_id;
        pool_ica_info.host_connection_id = counterparty_version.host_connection_id;
        pool_ica_info.host_channel_id = counterparty_channel_id;
        pool_ica_info.ica_addr = counterparty_version.address;
    } else {
        withdraw_ica_info.ctrl_channel_id = ctrl_channel_id;
        withdraw_ica_info.ctrl_port_id = ctrl_port_id;
        withdraw_ica_info.host_connection_id = counterparty_version.host_connection_id;
        withdraw_ica_info.host_channel_id = counterparty_channel_id;
        withdraw_ica_info.ica_addr = counterparty_version.address;
    }

    if !pool_ica_info.ica_addr.is_empty()
        && !withdraw_ica_info.ica_addr.is_empty()
        && !POOLS.has(deps.storage, pool_ica_info.ica_addr.clone())
    {
        let pool_info = PoolInfo {
            ica_id: ica_id.clone(),
            admin: admin.clone(),
            ..Default::default()
        };

        POOLS.save(deps.storage, pool_ica_info.ica_addr.clone(), &pool_info)?;
    }

    INFO_OF_ICA_ID.save(
        deps.storage,
        ica_id.clone(),
        &(pool_ica_info, withdraw_ica_info, admin),
    )?;

    return Ok(Response::default());
}
