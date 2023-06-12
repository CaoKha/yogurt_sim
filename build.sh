#! /bin/sh
#
# compile rust to wasm
# cargo build --release --target wasm32-unknown-unknown
# wasm-bindgen --out-dir ./server/public/wasm/ --target web ./target/wasm32-unknown-unknown/release/yogurt_sim.wasm
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
echo "Installing wasm-pack"
cargo install wasm-pack
echo "Compiling wasm to js..."
BASEDIR=$(dirname "$0")
wasm-pack build $BASEDIR --release --target web --out-dir server/pkg
echo "Change directory to server/"
cd $BASEDIR/server
echo "Installing packages..."
yarn install
echo "Building nextjs server..."
yarn run build
echo "Finished building!"
