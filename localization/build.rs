//! Build script for the `localization` crate.
//!
//! This watches the translation root so procedural macros are re-expanded when
//! translation files change.

use std::{env, path::PathBuf};

fn readme_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let package_readme = manifest_dir.join("README.md");
    if package_readme.exists() {
        package_readme
    } else {
        manifest_dir.join("..").join("README.md")
    }
}

fn main() {
    let readme = readme_path();
    println!("cargo:rerun-if-changed={}", readme.display());
    println!("cargo:rustc-env=LOCALIZATION_README={}", readme.display());

    let path = env::var("LOCALIZATION_ROOT").unwrap_or_else(|_| "./translations".to_string());
    println!("cargo:rerun-if-changed={}", path);
}
