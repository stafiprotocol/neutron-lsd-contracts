#!/usr/bin/env bash

config_decimals() {
  echo "-------------------------- config decimal -------------------------------------"

  msg=$(printf '{
    "config_decimals": {
      "remote_denom": "uatom",
      "decimals": 6
    }
  }')

  # echo "the msg is: $msg"
  tx_result="$(
    neutrond tx wasm execute "$contract_address" "$msg" \
      --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
      --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
      --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx
  )"

  code="$(echo "$tx_result" | jq '.code')"
  if [[ "$code" -ne 0 ]]; then
    echo "Failed to init pool: $(echo "$tx_result" | jq '.raw_log')" && exit 1
  fi

}