# Localization

[![GitHub](https://img.shields.io/github/license/neodyland/localization)](https://github.com/neodyland/localization/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/localization)](https://crates.io/crates/localization)
[![crates.io](https://img.shields.io/crates/d/localization)](https://crates.io/crates/localization)

localization is a lightweight localization implementation written in Rust.

Easy, error on compile time, zero runtime dependency.

## Getting Started

### Install

```toml
[dependencies]
localization = "0.1.2"
[build-dependencies]
localization-build = "0.1.2"
```

### Create files

```json5
// translations/en-US/default.json
{
    "hello": "Hello {{name}}, you are {{age}} years old!"
}
```

```rust
// build.rs
fn main() {
    localization_build::set_root("./translations");
    localization_build::set_default_locale("en-US");
}
```

```rust
// main.rs
fn main() {
    let name = "John";
    let age = 42;
    let s = t!("en-US","default:hello", name, age);
    println!("{}", s);
    // output: Hello John, you are 42 years old!
}
```

## Documentation

The documentation is available on [docs.rs](https://docs.rs/localization).

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
