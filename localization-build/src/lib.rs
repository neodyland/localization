/// Sets the root directory for the localization files
/// default: ./localization
pub fn set_root(root: &'static str) {
    println!("cargo:rustc-env=LOCALIZATION_ROOT={root}")
}

/// Sets the default locale to use as fallback
/// default: en-US
pub fn set_default_locale(locale: &'static str) {
    println!("cargo:rustc-env=LOCALIZATION_DEFAULT={locale}")
}