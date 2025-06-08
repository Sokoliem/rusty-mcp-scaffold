fn main() {
    // Get rustc version
    let output = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc");
    
    let version = String::from_utf8_lossy(&output.stdout);
    println!("cargo:rustc-env=RUSTC_VERSION={}", version.trim());
}
