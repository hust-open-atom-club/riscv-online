# riscv-online

Online RISC-V assembly development tools.

# Setup
This project is a RISC-V online disassembly website written in Rust WebAssembly and JavaScript.

## The Rust Toolchain
[You will need the standard Rust toolchain, including rustup, rustc, and cargo.](https://www.rust-lang.org/tools/install)

## wasm-pack
[wasm-pack is your one-stop shop for building, testing, and publishing Rust-generated WebAssembly.](https://rustwasm.github.io/wasm-pack/installer/)

## npm
[npm is a package manager for JavaScript.](https://www.npmjs.com/get-npm)

# Getting Started

## Building from Source

1. Download the latest source code.

```shell
git clone git@github.com:hust-open-atom-club/riscv-online.git
```

2. Compile our Rust sources into a WebAssembly.

```shell
cd wasm-riscv-online/www
wasm-pack build
```

3. Ensure that the local development server and its dependencies are installed.

```shell
npm install
```

4. Run

```shell
npm run start
```

5. Navigate your Web browser to `http://localhost:8080/`. 
