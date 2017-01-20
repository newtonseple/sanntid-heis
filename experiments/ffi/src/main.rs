extern crate libc;

//#[link(name = "code", kind = "static")]
extern "C" {
    fn add_two(x: i32) -> i32;
}

fn r_add_two(a: i32) -> i32 {
	unsafe { add_two(a) }
}

fn main() {
	let a = 2;
    println!("{}", r_add_two(a));
    println!("{}", a);
}
