//! Build script for package documentation.

use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let package_readme = manifest_dir.join("README.md");
    let readme = if package_readme.exists() {
        package_readme
    } else {
        manifest_dir.join("..").join("README.md")
    };

    println!("cargo:rerun-if-changed={}", readme.display());
    println!("cargo:rustc-env=LOCALIZATION_README={}", readme.display());
}
