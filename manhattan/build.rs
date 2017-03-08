extern crate gcc;

fn main() {

    gcc::Config::new()
		.flag("-std=c99")
		.file("src/hardware_io/rust_driver/c_driver/code.c") //remove this at some point in future
		.file("src/hardware_io/rust_driver/c_driver/io.c")
		.file("src/hardware_io/rust_driver/c_driver/elev.c")
		.file("src/hardware_io/rust_driver/c_driver/con_load.h")
		//.file("src/hardware_io/c_driver/test_run.c")
		.compile("libc_driver.a");
    //gcc::compile_library("libdriver.a", &["src/hardware_io/c_driver/code.c",
    //									  "src/hardware_io/c_driver/elev.c"]);
    println!("cargo:rustc-link-lib=comedi");
}