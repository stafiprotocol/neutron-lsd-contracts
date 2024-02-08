#!/bin/ash
# shellcheck shell=dash
# See https://www.shellcheck.net/wiki/SC2187
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

export PATH="$PATH:/root/.cargo/bin"

# Suffix for non-Intel built artifacts
MACHINE=$(uname -m)
SUFFIX=${MACHINE#x86_64}
SUFFIX=${SUFFIX:+-$SUFFIX}

# Debug toolchain and default Rust version
rustup toolchain list
cargo --version

# Prepare artifacts directory for later use
mkdir -p debug_artifacts

# Delete previously built artifacts. Those can exist if the image is called
# with a cache mounted to /target. In cases where contracts are removed over time,
# old builds in cache should not be contained in the result of the next build.
rm -f /target/wasm32-unknown-unknown/release/*.wasm
rm -f /target/wasm32-unknown-unknown/debug/*.wasm

# There are two cases here
# 1. The contract is included in the root workspace (eg. `cosmwasm-template`)
#    In this case, we pass no argument, just mount the proper directory.
# 2. Contracts are excluded from the root workspace, but import relative paths from other packages (only `cosmwasm`).
#    In this case, we mount root workspace and pass in a path `docker run <repo> ./contracts/hackatom`

export RUSTFLAGS="-C link-arg=-s" 

for CONTRACT in contracts/*; do
  (
    echo "Building $CONTRACT..."
    cd $CONTRACT
    cargo build --target-dir=/target --lib --target=wasm32-unknown-unknown --locked
  )
done

echo "Optimizing artifacts ..."
for WASM in /target/wasm32-unknown-unknown/debug/*.wasm; do
  OUT_FILENAME=$(basename "$WASM" .wasm)${SUFFIX}.wasm
  echo "Optimizing $OUT_FILENAME ..."
  # --signext-lowering is needed to support blockchains runnning CosmWasm < 1.3. It can be removed eventually
  wasm-opt -Os --signext-lowering "$WASM" -o "debug_artifacts/$OUT_FILENAME"
done

echo "Post-processing artifacts..."
(
  cd debug_artifacts
  # create hashes
  sha256sum -- *.wasm | tee checksums.txt
)

echo "done"