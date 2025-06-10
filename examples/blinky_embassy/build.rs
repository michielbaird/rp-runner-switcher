//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let rp2040 = std::env::var("CARGO_FEATURE_RP2040").is_ok();
    let rp2350 = std::env::var("CARGO_FEATURE_RP2350").is_ok();

    if rp2040 && rp2350 {
        panic!("Features `rp2040` and `rp2350` are mutually exclusive, please enable only one.");
    }
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let memory_path = if rp2040 {
        "memory_rp2040.x"
    } else if rp2350 {
        "memory_rp2350.x"
    } else {
        panic!("Must enable either rp2040 or rp2350 feature");
    };

    let memory_bytes = fs::read(memory_path).expect("Failed to read memory.x");
    let out_path = out.join("memory.x");
    File::create(&out_path)
        .expect("Failed to create memory.x")
        .write_all(&memory_bytes)
        .expect("Failed to write memory.x");

    // Tell rustc to pass the linker search path
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=memory_rp2350.x");
    println!("cargo:rerun-if-changed=memory_rp2040.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    if rp2040 {
        println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
    }
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
