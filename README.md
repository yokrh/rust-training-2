# Rust training

Rust training

## Workflow

my work log.

### Install

https://www.rust-lang.org/ja/learn/get-started

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# error: cannot install while Rust is installed
```
it seems I have tried before...


```sh
# confirm Rust and Cargo, which is a build tool of Rust, are installed
cargo --version
```

### Project setup

```sh
cargo new hello-rust
hello-rust
```

## Main command

```sh
cargo build
cargo run
cargo test
cargo doc
```

```sh
cargo run < src/input.txt 
```

## Reference

* sample code of stdin
https://qiita.com/penguinshunya/items/cd96803b74635aebefd6

