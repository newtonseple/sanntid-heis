extern crate gcc;

fn main() {

    gcc::Config::new()
		.flag("-std=gnu11")
		.file("src/hardware_io/rust_driver/c_driver/io.c")
		.file("src/hardware_io/rust_driver/c_driver/elev.c")
		.compile("libc_driver.a");
    println!("cargo:rustc-link-lib=comedi");
}