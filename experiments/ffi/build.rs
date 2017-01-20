/**/
// build.rs

// Bring in a dependency on an externally maintained `gcc` package which manages
// invoking the C compiler.

extern crate gcc;

fn main() {
    gcc::compile_library("libcode.a", &["src/code.c"]);
}

// build.rs
/*

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("gcc").args(&["src/code.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/code.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libcode.a", "code.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=code");
}*/
