#!/bin/bash
set -e

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# 安装 wasm-pack
cargo install wasm-pack

# 编译 wasm
cd ..
wasm-pack build

# 构建前端
cd www
npm install
npm run build 