fn main() {
    if std::path::Path::new("goduno.dll").exists() {
        std::fs::remove_file("goduno.dll").expect("Failed to delete the old library");
    }
    #[cfg(debug_assertions)]
    std::fs::copy("target/debug/goduno.dll", "goduno.dll").expect("Failed to copy the library");
    #[cfg(not(debug_assertions))]
    std::fs::copy("target/release/goduno.dll", "goduno.dll").expect("Failed to copy the library");
}
