#! /bin/sh
#
# compile rust to wasm
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./server/src/utils/wasm/ --target web ./target/wasm32-unknown-unknown/release/yogurt_sim.wasm
