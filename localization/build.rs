//! Build script for the `localization` crate.
//!
//! This watches the translation root so procedural macros are re-expanded when
//! translation files change.

use std::env;

fn main() {
    let path = env::var("LOCALIZATION_ROOT").unwrap_or_else(|_| "./translations".to_string());
    println!("cargo:rerun-if-changed={}", path);
}
