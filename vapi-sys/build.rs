extern crate bindgen;

use pkg_config::find_library;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system varnishapi and m
    // shared libraries.
    println!("cargo:rustc-link-lib=varnishapi");
    println!("cargo:rustc-link-lib=m");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Find varnish includes via pkg-config
    let lib = match find_library("varnishapi") {
        Ok(l) => l,
        Err(e) => {
            panic!("Error finding 'varnishapi' via pkg-config: {}", e);
        }
    };

    let clang_args: Vec<String> = lib
        .include_paths
        .into_iter()
        .map(|p| format!("-I{}", p.to_string_lossy()))
        .collect();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .allowlist_type("VSC_.*")
        .allowlist_function("VSC_.*")
        .allowlist_var("VSC_.*")
        .allowlist_type("vsm_.*")
        .allowlist_type("VSM_.*")
        .allowlist_function("VSM_.*")
        .allowlist_var("VSM_.*")
        .allowlist_type("VSL.*")
        .allowlist_function("VSL.*")
        .allowlist_var("VSL.*")
        .clang_args(clang_args)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let gen_path = PathBuf::new().join("generated");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    bindings
        .write_to_file(gen_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
