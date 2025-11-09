[README.en.md](https://github.com/user-attachments/files/23439889/README.en.md)
<div align="center">

# RISC-V Online 🚀

[![License](https://img.shields.io/badge/License-Mulan%20PSL%20v2-blue.svg)](http://license.coscl.org.cn/MulanPSL2)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?logo=javascript&logoColor=black)](https://developer.mozilla.org/en-US/docs/Web/JavaScript)
<!-- [![Build Status](https://travis-ci.org/hust-open-atom-club/riscv-online.svg?branch=main)](https://travis-ci.org/hust-open-atom-club/riscv-online) -->

**🌐 Web-based RISC-V Disassembly Tool**

[🇨🇳 中文](README.md) | [🇺🇸 English](README.en.md) | [🔧 Contributing Guide](CONTRIBUTING.md)

</div>

## ✨ Project Overview

**RISC-V Online** is a zero-install, browser-based RISC-V disassembler powered by WebAssembly.  
Built for RISC-V developers, students and researchers.

**15-second pitch:** Paste hex machine code, instantly get human-readable RISC-V assembly — no setup, all in your browser!

## 🎯 Key Features

- **🔧 Instant Disassembly** – 16/32-bit RISC-V instruction support  
- **🌐 Zero-Install** – Runs in any modern browser via WebAssembly  
- **⚡ Blazing Fast** – Rust core delivers millisecond-level response  
- **📱 Cross-Platform** – Works on desktop, tablet and mobile  
- **🎨 Intuitive UI** – Clean input/output, multiple format support  
- **🔍 Smart Parsing** – Auto-detects instruction length, GNU objdump-compatible output  

## 📋 Supported Instruction Set Extensions

| Extension | Support Status |
|-----------|---------------|
| RV32I     | ✅            |
| RV64I     | ✅            |
| RVM       | ✅            |
| RVC       | ✅            |
| RV32F     | ✅            |
| RV64F     | ✅            |
| RVZicsr   | ✅            |
| RV32A     | ✅            |
| RV64D     | ❌            |
| RVB       | ❌            |

## 🚀 Quick Start

### Local Development

```bash
# Clone the repo
git clone https://github.com/hust-open-atom-club/riscv-online.git
cd riscv-online

# Build the WebAssembly module
cd wasm-riscv-online
wasm-pack build

# Install deps & start dev server
cd www
npm install
npm run start
```

Open `http://localhost:8080` and you’re ready to disassemble!

## 🛠️ Development Environment

| Tool      | Version   | Purpose                     |
|-----------|-----------|-----------------------------|
| Rust      | ≥ 1.56.0  | Compile to WebAssembly      |
| wasm-pack | ≥ 0.10.0  | Rust → WASM build tool      |
| Node.js   | ≥ 16.0.0  | Frontend dev server         |
| npm       | ≥ 8.0.0   | Package manager             |

## 🤝 Contributing

We welcome all kinds of contributions!  
Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Quick Ways to Help

- 🐛 **Report Bugs** – [Open an Issue](https://github.com/hust-open-atom-club/riscv-online/issues/new)  
- ✨ **Add Features** – Fork → Hack → Pull Request  
- 📝 **Improve Docs** – Better README, examples, tutorials  
- 🧪 **Add Tests** – Increase test coverage  

## 📜 License

This project is open source under the **Mulan Permissive Software License, Version 2**.  
See [LICENSE](LICENSE) for the full text.
