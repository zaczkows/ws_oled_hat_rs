use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=native/bcm2835.h");
    println!("cargo:rerun-if-changed=native/bcm2835.c");

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("native/bcm2835.c")
        .flag("-w")
        .flag("-fPIC")
        .define("BCM2835_NO_DELAY_COMPATIBILITY", None)
        .static_flag(true)
        .compile("bcm2835");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("native/bcm2835.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        /*.whitelist_function("bcm2835_init")
        .whitelist_function("bcm2835_close")
        .whitelist_function("bcm2835_gpio_sfel")
        .whitelist_function("bcm2835_gpio_write")
        // spi
        .whitelist_function("bcm2835_spi_transfer")
        .whitelist_function("bcm2835_spi_begin")
        .whitelist_function("bcm2835_spi_end")*/
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
