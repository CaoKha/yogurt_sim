# yogurt_sim
[![CI](https://github.com/CaoKha/yogurt_sim/workflows/CI/badge.svg)](https://github.com/CaoKha/yogurt_sim/actions/workflows/ci.yaml)

A yogurt simulation (currently in development phase)
![ohaio](https://media.tenor.com/G78em4XTdjwAAAAd/yogurt.gif)

## Prerequisites
Installing Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```
Installing wasm-pack
```bash
cargo install wasm-pack
```

## Building
Run these commands:
```bash
chmod +x build.sh
./build.sh
```
These commands will generate a `pkg/` folder containing wasm and js files for the nextjs server.
When a code is changed in lib.rs, remember to run 
```bash
./build.sh

```

## Examples
```bash
cargo run --example ball_game
```

If you want to run on a website (Nextjs), make sure you have Nodejs (LTS) and Yarn installed, then run:  
```
cd server && yarn dev
```

