fn main() {
    println!("cargo:rerun-if-changed=clib/*"); // Re-run build script if anything in the clib directory changes
    println!("cargo:rustc-link-lib=hello"); // Link against the 'myclib' library
    println!("cargo:rustc-link-lib=raylib"); // Link against the 'myclib' library
    println!("cargo:rustc-link-search=clib"); // Search for library in the 'clib' directory
}
