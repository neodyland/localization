# Localization

[![GitHub](https://img.shields.io/github/license/neodyland/localization)](https://github.com/neodyland/localization/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/localization)](https://crates.io/crates/localization)
[![crates.io](https://img.shields.io/crates/d/localization)](https://crates.io/crates/localization)

localization is a lightweight localization implementation written in Rust.

Embeds translation data into the binary at build time, so it runs faster.

## Getting Started

### Install

```
cargo add localization localization-build
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
}
```

```rust
fn main() {
// main.rs
    let name = "John";
    let age = 42;
    let s = t!("ja-JP","default:hello", name, age);
    println!("{}", s);
    // output: Hello John, you are 42 years old!
}
```

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
