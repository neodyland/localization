fn main() {
    let path = std::env::var("LOCALIZATION_ROOT").unwrap_or("./translations".to_string());
    println!("cargo:rerun-if-changed={}", path);
}
