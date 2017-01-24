extern crate gcc;

fn main() {
    gcc::compile_library("libc_foo.a", &["src/c_foo.c"]);
}