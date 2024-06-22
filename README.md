## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

在我们的课程中，新的项目会使用 `rust-learn-days/template` 模版生成基本的代码：

```bash
cargo generate tyr-rust-bootcamp/template
```

### 安装 just

just 是一个类似Makefile 的工具。

```bash
brew install just
```

安装成功后运行 `just init` 即可初始化环境。


### 安装 tokei

tokei 是一个代码统计工具。

```bash
brew install tokei
```
