# README

* basic-data-structure
* tutorial
* ownership_and_move

## install

```bash
# 安装 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 验证是否安装成功
# rustc 是 rust 编译器
rustc --version
```

也可以安装指定版本的 rust：

```bash
# 安装 Rust 1.70.0
rustup toolchain install 1.70.0

# 设置默认版本
rustup default 1.70.0
```

## VSCode plugins

* rust-analyzer
* Rust Syntax
* Even Better TOML

## Setup

```bash
# 文件夹不存在
cargo new hello-world
cd hello-world

# 文件夹已存在
cargo init
```

## Build

```bash
cargo build --release
```

## Run

```bash
# 查看命令行参数
cargo run -- --help

# 运行
cargo run
```

## Clean

```bash
# 清理生成的文件
cargo clean
```

## Unit Test

```bash
cargo test
```
