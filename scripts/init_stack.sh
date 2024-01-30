#!/usr/bin/env bash
init_stack() {
  echo "--------------------------instantiate stake manager-------------------------------------"

  msg=$(printf '{
    "lsd_token_code_id": %d
}' "$lsd_code_id")

  contract_address=$(neutrond tx wasm instantiate "$stake_manager_code_id" "$msg" \
    --from "$ADDRESS_1" --admin "$ADMIN" -y --chain-id "$CHAIN_ID_1" \
    --output json --broadcast-mode=sync --label "init" \
    --keyring-backend=test --gas-prices 0.0025untrn --gas auto \
    --gas-adjustment 1.4 --home "$HOME_1" \
    --node "$NEUTRON_NODE" 2>/dev/null |
    wait_tx | jq -r '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')
  echo "Contract address: $contract_address"

  echo "--------------Sent money to contract to pay fees---------------------------"

  tx_result="$(neutrond tx bank send demowallet1 "$contract_address" 10000000untrn \
    --chain-id "$CHAIN_ID_1" --home "$HOME_1" --node "$NEUTRON_NODE" \
    --keyring-backend=test -y --gas-prices 0.0025untrn \
    --broadcast-mode=sync --output json | wait_tx)"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to send money to contract: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi
}
