# 欢迎贡献

感谢您对 RISC-V 在线反汇编工具项目的关注！这是一个基于 WebAssembly 的 RISC-V 反汇编器，支持多种指令集扩展。

## 项目概述

本项目是一个 Web 端 RISC-V 反汇编工具，将十六进制机器码转换为人类可读的汇编语言。系统采用 Rust 后端编译为 WebAssembly，配合 JavaScript 前端实现高性能的实时反汇编功能。README.md:6

支持的指令集包括：RV32I、RV64I、RVC（压缩指令）、RVF（浮点）和 RVZicsr（控制状态寄存器）。

## 开发环境设置

### 前置要求

1. **Rust 工具链**

- 安装 Rust：`curl --proto '=https' --tlsv1.2 -sSf <https://sh.rustup.rs> | sh`
- 安装 wasm-pack：`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

2. **Node.js 和 npm**

- Windows: 从 nodejs.org 下载安装
- macOS: `brew install node`
- Ubuntu: `apt install nodejs npm`

### 构建和运行

1. 克隆仓库：

```bash
git clone <git@github.com>:hust-open-atom-club/riscv-online.git  
cd riscv-online/wasm-riscv-online/www
```

2. 编译 Rust 到 WebAssembly：

```bash
wasm-pack build
```

3. 安装 JavaScript 依赖：

```bash
npm install
```

4. 启动开发服务器：

```bash
npm run start
```

开发服务器将在 `localhost:8080` 运行。

## 项目结构

项目采用双语言架构，主要目录结构如下：

- `wasm-riscv-online/src/`- Rust 源码
  - `lib.rs` - WebAssembly 接口
  - `decode/` - 指令解析模块
  - `asm/` - 汇编格式化模块
  - `riscv/` - 核心数据类型
- `wasm-riscv-online/www/` - Web 前端
  - `index.html` - 主界面
  - `index.js` - JavaScript 应用逻辑
  - `package.json` - Web 包配置

## 贡献指南

### 代码风格

1. **Rust 代码**
   - 使用  `cargo fmt` 格式化代码
   - 使用 `cargo clippy` 进行代码检查
   - 遵循 Rust 官方编码规范
2. **JavaScript 代码**
   - 使用 2 空格缩进
   - 使用分号结尾
   - 变量命名使用 camelCase

## 提交流程

1. **Fork 项目**

   - 在 GitHub 上 fork 此仓库
   - 克隆您的 fork 到本地
2. **创建分支**

```bash
git checkout -b feature/your-feature-name
```

3. **开发和测试**

   - 确保代码能够正确编译
   - 测试新功能或修复
   - 验证 Web 界面正常工作
4. **提交代码**

```bash
git add .  
git commit -m "类型: 简短描述您的更改"
```

5. **推送和创建 PR**

```bash
git push origin feature/your-feature-name
```

然后在 GitHub 上创建 Pull Request

## 提交信息格式

使用以下格式编写提交信息：

- `feat: 添加新功能`
- `fix: 修复 bug`
- `docs: 更新文档`
- `style: 代码格式调整`
- `refactor: 重构代码`
- `test: 添加或修改测试`
- `build: 构建系统相关更改`

### 问题报告

报告 bug 时请包含：

1. **环境信息**
   - 操作系统和版本
   - 浏览器类型和版本
   - Rust 和 Node.js 版本
2. **重现步骤**
   - 详细的操作步骤
   - 输入的十六进制代码
   - 期望的输出 vs 实际输出
3. **错误信息**
   - 控制台错误信息
   - 浏览器开发者工具信息

### 功能请求

提出新功能时请说明：

1. **功能描述** - 清楚地描述所需功能
2. **使用场景** - 解释为什么需要此功能
3. **实现建议** - 如果有技术方案，请分享

## 测试

目前项目包含基础测试框架。贡献代码时请：

1. **运行现有测试**

```bash
cargo test
```

2. **添加新测试**

   - 为新功能添加单元测试
   - 确保测试覆盖关键路径
3. **手动测试**

   - 在浏览器中测试 Web 界面
   - 验证各种 RISC-V 指令类型

## 许可证

本项目使用木兰宽松许可证，第 2 版（Mulan PSL v2）。LICENSE:1-2

贡献代码即表示您同意在相同许可证下发布您的贡献。
