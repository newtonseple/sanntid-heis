extern crate gcc;

fn main() {

    gcc::Config::new()
		.flag("-std=gnu11")
		.file("src/hardware_io/rust_driver/c_driver/code.c") //remove this at some point in future
		.file("src/hardware_io/rust_driver/c_driver/elev.c")
		//.file("src/hardware_io/c_driver/test_run.c")
		.compile("libc_driver.a");
    //println!("cargo:rustc-link-lib=comedi");
}