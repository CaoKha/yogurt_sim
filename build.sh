#! /bin/sh
#
# echo "Installing Rust..."
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# echo "Installing wasm-pack"
# cargo install wasm-pack
echo "Compiling wasm to js..."
BASEDIR=$(dirname "$0")
wasm-pack build $BASEDIR --release --target web --out-dir server/pkg
echo "Change directory to server/"
cd $BASEDIR/server
echo "Remove old cache"
rm -r .next
echo "Installing packages..."
yarn install
echo "Building nextjs server..."
yarn run build
echo "Starting server..."
yarn run start
