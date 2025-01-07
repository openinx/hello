extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.header = Some(
        String::from("// Licensed under the Apache License, Version 2.0 (the \"License\")"));
    config.language = cbindgen::Language::C;

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("src/c.h");
}