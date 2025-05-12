# riscv-online

Online RISC-V assembly development tools.

## Setup
This project is a RISC-V online disassembly website written in Rust WebAssembly and JavaScript.

### Rust Toolchain
[Rust toolchain](https://www.rust-lang.org/tools/install) including `rustup`, `rustc`, and `cargo`.

Install using rustup(Recommended)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### wasm-pack 
[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) is your one-stop shop for building, testing, and publishing Rust-generated WebAssembly.

Install on Unix, Linux, MacOS with curl
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
Install from source on Windows and any platform with cargo
```shell
cargo install wasm-pack
```

### npm
[npm](https://www.npmjs.com/get-npm) is a package manager for JavaScript.

To set up `npm`, [`nodejs`](https://nodejs.org/zh-cn) is required.

Install on Windows with executeable file download from [nodejs homepage](https://nodejs.org/zh-cn)

Install on Macos with [brew](https://brew.sh/)

```bash
brew install node
```

Install on Ubuntu with apt
```
sudo apt install nodejs npm
```

Or with other system package manager you like

## Selfhost

### Stepping to build

Download the latest source code.

```shell
git clone git@github.com:hust-open-atom-club/riscv-online.git
```

Compile rust code into WebAssembly

```shell
cd wasm-riscv-online/www
wasm-pack build
```

Install JavaScript package

```shell
npm install
```

Start local http server

```shell
npm run start
```

Navigate your Web browser to `http://localhost:8080/`. 
