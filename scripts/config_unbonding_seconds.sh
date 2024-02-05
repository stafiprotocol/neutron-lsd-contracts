#!/usr/bin/env bash

config_unbonding_seconds() {
  echo "-------------------------- config unbonding seconds -------------------------------------"

  msg=$(printf '{
  "config_unbonding_seconds": {
    "remote_denom": "uatom",
    "unbonding_seconds": 20
  }
}')
  # echo "config pool msg is: $msg"
  tx_result="$(neutrond tx wasm execute "$contract_address" "$msg" \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to config unbonding seconds: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi

  query="$(printf '{"unbonding_seconds": {"remote_denom": "uatom"}}')"
  echo "------------------------ unbonding seconds info after config ------------------------"
  neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq
}
