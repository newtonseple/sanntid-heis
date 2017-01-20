extern crate gcc;

fn main() {

	gcc::Config::new()
		.flag("-std=c99")
		.file("src/driver/c_driver/code.c") //remove this at some point in future
		.file("src/driver/c_driver/io.c")
		.file("src/driver/c_driver/elev.c")
		.file("src/driver/c_driver/test_run.c")
		.compile("libfoo.a");
    //gcc::compile_library("libdriver.a", &["src/driver/c_driver/code.c",
    //									  "src/driver/c_driver/elev.c"]);
    println!("cargo:rustc-link-lib=comedi");
}