# yogurt_sim
[![Build Status](https://github.com/CaoKha/yogurt_sim/actions/workflows/ci.yaml/badge.svg)]

A yogurt simulation (currently in development phase)
![ohaio](https://media.tenor.com/G78em4XTdjwAAAAd/yogurt.gif)

## Prerequisites
Run this command inside yogurt_sim:
```bash
mkdir .cargo
touch .cargo/config.toml
```
Add these line into config.toml file:
```
# Add the contents of this file to `config.toml` to enable "fast build" configuration. Please read the notes below.
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"

# NOTE: For maximum performance, build using a nightly compiler
# If you are using rust stable, remove the "-Zshare-generics=y" below.

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-Clink-arg=-fuse-ld=lld", "-Zshare-generics=y"]

# NOTE: you must install [Mach-O LLD Port](https://lld.llvm.org/MachO/index.html) on mac. you can easily do this by installing llvm which includes lld with the "brew" package manager:
# `brew install llvm`
[target.x86_64-apple-darwin]
rustflags = [
    "-C",
    "link-arg=-fuse-ld=/usr/local/opt/llvm/bin/ld64.lld",
    "-Zshare-generics=y",
]

[target.aarch64-apple-darwin]
rustflags = [
    "-C",
    "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld",
    "-Zshare-generics=y",
]

[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=n"]

# Optional: Uncommenting the following improves compile times, but reduces the amount of debug info to 'line number tables only'
# In most cases the gains are negligible, but if you are on macos and have slow compile times you should see significant gains.
#[profile.dev]
#debug = 1

```
or for simplicity, you can go to bevy_book and do all of their prerequisites


## Compilation
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

## Testing
If you want to test on PC instead of wasm. Run:
```bash
cargo run
```

If you want to run on a website (Nextjs), make sure you have Nodejs and Yarn installed, then run:  
```
cd server && yarn start
```

