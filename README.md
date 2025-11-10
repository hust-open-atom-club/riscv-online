<div align="center">  
  
# RobustOne Online 🚀  
  
[![License](https://img.shields.io/badge/License-Mulan%20PSL%20v2-blue.svg)](http://license.coscl.org.cn/MulanPSL2)  
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=white)](https://webassembly.org/)  
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)  
[![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?logo=javascript&logoColor=black)](https://developer.mozilla.org/en-US/docs/Web/JavaScript)  
<!-- [![Build Status](https://travis-ci.org/hust-open-atom-club/robustone-online.svg?branch=main)](https://travis-ci.org/hust-open-atom-club/robustone-online)   -->
  
**🌐 在线 RISC-V 汇编反汇编工具 | Web-based RISC-V Disassembly Tool**  
  
[🇨🇳 中文](README.md) | [🇺🇸 English](README.en.md) | [🔧 贡献指南](CONTRIBUTING.md)  
  
</div>  
  
## ✨ 项目简介  
  
RobustOne Online 是一款基于 WebAssembly 的在线 RISC-V 汇编反汇编工具，专为 RISC-V 开发者、学习者和研究人员设计。通过将 Rust 编译为 WebAssembly，我们实现了高性能的实时反汇编功能，让您可以在浏览器中直接使用，无需任何安装或配置。
  
**15秒快速了解：** 输入十六进制机器码，立即获得 RISC-V 汇编指令 — 无需安装，浏览器即用！
  
## 🎯 核心特性  
  
- **🔧 即时反汇编** - 支持 16 位和 32 位 RISC-V 指令集
- **🌐 零安装使用** - 基于 WebAssembly，在浏览器中直接运行  
- **⚡ 高性能处理** - Rust 编写的核心解析引擎，毫秒级响应  
- **📱 跨平台支持** - 支持所有现代浏览器，移动端友好  
- **🎨 直观界面** - 清晰的输入输出，支持多种输入格式  
- **🔍 智能解析** - 自动识别指令长度，支持 GNU objdump 输出格式  

## 📋 支持的指令集扩展

| 指令集扩展 | 支持状态 |
|-----------|---------|
| RV32I     | ✅      |
| RV64I     | ✅      |
| RVM       | ✅      |
| RVC       | ✅      |
| RV32F     | ✅      |
| RV64F     | ✅      |
| RVZicsr   | ✅      |
| RV32A     | ✅      |
| RV64A     | ✅      |
| RV128A    | ✅      |
| RV64D     | ✅      |
| RVB       | ✅      |

## 🚀 快速开始
  
### 本地部署  
  
```bash  
# 克隆项目
git clone https://github.com/hust-open-atom-club/robustone-online.git  
cd robustone-online  
  
# 构建 WebAssembly 模块
cd wasm-riscv-online  
wasm-pack build  
  
# 安装依赖并启动开发服务器
cd www  
npm install  
npm run start
```

访问 `http://localhost:8080` 即可使用。

## 🛠️ 开发环境  
  
### 系统要求  
  
| 工具 | 版本要求 | 用途 |  
|------|----------|------|  
| Rust | >= 1.56.0 | WebAssembly 编译 |  
| wasm-pack | >= 0.10.0 | Rust 到 WASM 构建工具 |  
| Node.js | >= 16.0.0 | 前端开发服务器 |  
| npm | >= 8.0.0 | 包管理器 |  

## 🤝 贡献指南

我们欢迎所有形式的贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

### 快速贡献  
  
1. **🐛 报告问题** - [创建 Issue](https://github.com/hust-open-atom-club/robustone-online/issues/new)  
2. **✨ 提交功能** - Fork → 修改 → Pull Request  
3. **📝 完善文档** - 改进 README 或添加示例  
4. **🧪 添加测试** - 提高代码覆盖率  

## 📜 许可证

本项目采用 木兰宽松许可证第 2 版 开源。
