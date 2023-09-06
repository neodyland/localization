pub fn set_root(root: &'static str) {
    println!("cargo:rustc-env=LOCALIZATION_ROOT={root}")
}
