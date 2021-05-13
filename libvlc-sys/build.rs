extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;

fn main() {
    pkg_config::probe_library("libvlc").unwrap();

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    let src = [
        "src/helper.c",
    ];
    let mut builder = cc::Build::new();
    builder.files(src.iter()).include("libvlc").compile("vlchelper");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        .array_pointers_in_arguments(true)
        .derive_default(true)
        .default_enum_style(EnumVariation::NewType { is_bitfield: true })
        .detect_include_paths(true)
        .size_t_is_usize(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
