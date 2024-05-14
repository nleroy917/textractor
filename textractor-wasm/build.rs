use std::env;

fn main() {
    // Set environment variables for clang and llvm-ar
    let clang_path = "/opt/homebrew/opt/llvm/bin/clang";
    let llvm_ar_path = "/opt/homebrew/opt/llvm/bin/llvm-ar";

    // Print cargo instructions to set environment variables
    println!("cargo:rustc-env=CC={}", clang_path);
    println!("cargo:rustc-env=AR={}", llvm_ar_path);
    
    // Add llvm bin directory to PATH
    let llvm_bin_path = "/opt/homebrew/opt/llvm/bin";
    let path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", llvm_bin_path, path);
    println!("cargo:rustc-env=PATH={}", new_path);

    // Optionally, rerun the build script if these files change
    println!("cargo:rerun-if-changed=build.rs");
}
