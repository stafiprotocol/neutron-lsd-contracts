#!/usr/bin/env bash

redeem_token_for_share() {

  echo "-------------------------- redeem token for share -------------------------------------"

  query="$(printf '{"pool_info": {"pool_addr": "%s"}}' "$pool_address")"
  tokens=$(neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq ".data.share_tokens")

  # era_update round 1
  redeem_msg=$(printf '{
  "redeem_token_for_share": {
    "pool_addr": "%s",
    "tokens": %s
  }
}' "$pool_address" "$tokens")

  echo "redeem msg: $redeem_msg"
  tx_result="$(neutrond tx wasm execute "$contract_address" "$redeem_msg" \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --amount 2000000untrn \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to redeem msg: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi

  print_wait_msg 15 "Waiting 15 seconds for redeem (sometimes it takes a lot of time)…"

  query="$(printf '{"pool_info": {"pool_addr": "%s"}}' "$pool_address")"
  echo "pool_info is: "
  echo "$query"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq

}

process_era() {
  echo "-------------------------- era update -------------------------------------"
  # era_update round 1
  era_update_msg=$(printf '{
  "era_update": {
    "pool_addr": "%s"
  }
}' "$pool_address")

  tx_result="$(neutrond tx wasm execute "$contract_address" "$era_update_msg" \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --amount 2000000untrn \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to era_update msg: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi

  print_wait_msg 10 "Waiting 10 seconds for era_update (sometimes it takes a lot of time)…"

  echo "query ica atom balance"
  gaiad query bank balances "$pool_address" --node "$GAIA_NODE" --output json | jq

  query="$(printf '{"pool_info": {"pool_addr": "%s"}}' "$pool_address")"
  echo "pool_info is: "
  echo "$query"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq
  print_wait_msg 40 "Waiting 40 seconds"

  echo "-------------------------- era stake -------------------------------------"
  # era_bond round 1
  bond_msg=$(printf '{
  "era_stake": {
    "pool_addr": "%s"
  }
}' "$pool_address")

  tx_result="$(neutrond tx wasm execute "$contract_address" "$bond_msg" \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --amount 2000000untrn \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to era_bond msg: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi

  print_wait_msg 15 "Waiting 15 seconds for era_bond (sometimes it takes a lot of time)…"

  gaiad query staking delegations "$pool_address" --node "$GAIA_NODE" --output json | jq

  gaiad query bank balances "$pool_address" --node "$GAIA_NODE" --output json | jq

  query="$(printf '{"pool_info": {"pool_addr": "%s"}}' "$pool_address")"
  echo "pool_info is: "
  echo "$query"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq

  echo "--------------------------collect withdraw -------------------------------------"
  print_wait_msg 40 "Waiting 40 seconds"

  # era_collect_withdraw_msg round 1
  era_collect_withdraw_msg=$(printf '{
  "era_collect_withdraw": {
    "pool_addr": "%s"
  }
}' "$pool_address")

  tx_result="$(neutrond tx wasm execute "$contract_address" "$era_collect_withdraw_msg" \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --amount 2000000untrn \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to era_collect_withdraw_msg msg: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi

  print_wait_msg 10 "Waiting 10 seconds for era_collect_withdraw_msg (sometimes it takes a lot of time)…"

  query="$(printf '{"pool_info": {"pool_addr": "%s"}}' "$pool_address")"
  echo "pool_info is: "
  echo "$query"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq

  echo "-------------------------- era restake -------------------------------------"

  era_rebond_msg=$(printf '{
  "era_restake": {
    "pool_addr": "%s"
  }
}' "$pool_address")

  tx_result="$(neutrond tx wasm execute "$contract_address" "$era_rebond_msg" \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --amount 2000000untrn \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to era_rebond_msg msg: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi

  print_wait_msg 10 "Waiting 10 seconds for era_rebond_msg (sometimes it takes a lot of time)…"

  gaiad query bank balances "$pool_address" --node "$GAIA_NODE" --output json | jq

  query="$(printf '{"pool_info": {"pool_addr": "%s"}}' "$pool_address")"
  echo "pool_info is: "
  echo "$query"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq

  echo "--------------------------era active-------------------------------------"
  print_wait_msg 40 "Waiting 40 seconds"

  # era_active_msg round 1
  era_active_msg=$(printf '{
  "era_active": {
    "pool_addr": "%s"
  }
}' "$pool_address")

  tx_result="$(neutrond tx wasm execute "$contract_address" "$era_active_msg" \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to era_active_msg msg: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi

  print_wait_msg 10 "Waiting 10 seconds for era_active_msg (sometimes it takes a lot of time)…"

  query="$(printf '{"pool_info": {"pool_addr": "%s"}}' "$pool_address")"
  echo "pool_info is: "
  echo "$query"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq

  query="$(printf '{"delegations": {"pool_addr": "%s", "sdk_greater_or_equal_v047": false}}' "$pool_address")"
  echo "the query is $query"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq
  
  query="$(printf '{"era_rate": {"pool_addr": "%s","era": 1}}' "$pool_address")"
  echo "the query is $query"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq

  # withdraw_addr="cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw"query_id=3
  echo "---------------------------------------------------------------"
}
