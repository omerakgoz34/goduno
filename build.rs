fn main() {
    let library = "goduno.dll";

    if std::path::Path::new(library).exists() {
        std::fs::remove_file(library).expect("Failed to delete the old library");
    }

    #[cfg(debug_assertions)]
    std::fs::copy(format!("target/debug/{}", library), library)
        .expect("Failed to copy the library");

    #[cfg(not(debug_assertions))]
    std::fs::copy(format!("target/release/{}", library), library)
        .expect("Failed to copy the library");

    match std::process::Command::new("llvm-strip")
        .args(&["-s", library])
        .output()
    {
        Ok(out) => {
            if !out.status.success() {
                let err = std::str::from_utf8(&out.stderr).unwrap();
                panic!("Command \"llvm-strip\" failed: {}", err);
            }
        }
        Err(err) => {
            panic!("Error running command \"llvm-strip\": {:#}", err);
        }
    }
}
