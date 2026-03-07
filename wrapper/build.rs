use std::env;
use std::path::PathBuf;

fn main() {
    // ---- Adjust these paths ----
    println!("cargo:rustc-link-search=native=../public/libraries/i80386linux_x64/release");
    println!("cargo:rustc-link-lib=dylib=XMPCore");
    println!("cargo:rustc-link-lib=dylib=XMPFiles");

    // Build C++ shim
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .file("xmp_shim.cpp")
        .include("../public/include")
        .define("WIN32", None)
        .define("WIN_ENV", None)
        .define("XMP_WinBuild", None)
        .define("XMP_StaticBuild", Some("0")) // critical
        .compile("xmp_shim");

    // Generate Rust bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();

    // Generate public C header
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate header")
        .write_to_file("include/xmp_wrapper.h");
}