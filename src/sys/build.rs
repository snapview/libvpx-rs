extern crate bindgen;
extern crate pkg_config as pkg;

use std::env;
use std::path::Path;

pub fn main() {
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = Path::new(&src_dir);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let ffi_header = src_dir.join("ffi.h");
    let ffi_rs = out_dir.join("ffi.rs");

    let libvpx = pkg::probe_library("vpx").unwrap();

    println!("rerun-if-changed={}", ffi_header.display());
    let mut b = bindgen::builder().header(ffi_header.to_str().unwrap())
        .no_unstable_rust()
        .generate_comments(false); // vpx comments have prefix /*!\

    for dir in &libvpx.include_paths {
        b = b.clang_arg(format!("-I{}", dir.display()));
        println!("rerun-if-changed={}", dir.display());
    }

    b.generate().unwrap()
        .write_to_file(ffi_rs).unwrap();
}
