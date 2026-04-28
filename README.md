# Localization

[![GitHub](https://img.shields.io/github/license/neodyland/localization)](https://github.com/neodyland/localization/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/localization)](https://crates.io/crates/localization)
[![crates.io](https://img.shields.io/crates/d/localization)](https://crates.io/crates/localization)

localization is a lightweight localization implementation written in Rust.

Easy, error on compile time, zero runtime dependency by default.

The `localization` crate supports `no_std` with the `alloc` feature. In that
configuration, `all!` uses `hashbrown` for its generated maps.

## Getting Started

### Install

```toml
[dependencies]
localization = "0.1"
[build-dependencies]
localization-build = "0.1"
```

### Create files

```json5
// translations/en-US/default.json
{
    "hello": "Hello {{name}}, you are {{age}} years old!"
}
```

```rust,ignore
// build.rs
fn main() {
    localization_build::set_root("./translations");
    localization_build::set_default_locale("en-US");
}
```

```rust,ignore
// main.rs
fn main() {
    let name = "John";
    let age = 42;
    let s = t!("en-US","default:hello", name, age);
    println!("{}", s);
    // output: Hello John, you are 42 years old!
}
```

## License

Distributed under the MIT License. See <https://github.com/neodyland/localization/blob/master/LICENSE> for more information.
