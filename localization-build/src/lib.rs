#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Sets the root directory that `localization` macros read at compile time.
///
/// Call this from your crate's `build.rs` before compilation. The path is
/// stored in the `LOCALIZATION_ROOT` environment variable and later consumed
/// by the procedural macros.
///
/// If this is not called, the default path is `./translations`.
///
/// ```rust
/// localization_build::set_root("./translations");
/// ```
pub fn set_root(root: &'static str) {
    println!("cargo:rustc-env=LOCALIZATION_ROOT={root}")
}

/// Sets the fallback locale used when a translation is missing.
///
/// Call this from your crate's `build.rs` to control which locale `t!`
/// falls back to when the requested locale does not contain a key.
///
/// If this is not called, the default locale is `"en-US"`.
///
/// ```rust
/// localization_build::set_default_locale("en-US");
/// ```
pub fn set_default_locale(locale: &'static str) {
    println!("cargo:rustc-env=LOCALIZATION_DEFAULT={locale}")
}
