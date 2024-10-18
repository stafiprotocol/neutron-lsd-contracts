#!/usr/bin/env bash
init_stack() {
  echo "--------------------------instantiate stake manager-------------------------------------"

  msg=$(printf '{
    "lsd_token_code_id": %d,
    "stack_fee_receiver": "%s"
}' "$lsd_code_id" "$ADDRESS_1")

  contract_address=$(neutrond tx wasm instantiate "$stake_manager_code_id" "$msg" \
    --from "$ADDRESS_1" --admin "$ADMIN" -y --chain-id "$CHAIN_ID_1" \
    --output json --broadcast-mode=sync --label "init" \
    --keyring-backend=test --gas-prices 0.0025untrn --gas auto \
    --gas-adjustment 1.4 --home "$HOME_1" \
    --node "$NEUTRON_NODE" 2>/dev/null |
    wait_tx | jq -r '.events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')
  echo "Contract address: $contract_address"

}
