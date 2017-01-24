extern crate libc;

extern fn c_btoi(c_bool: i32) -> i32;


#[repr(C)]
enum c_bool_t {
    FALSE,
    TRUE,
}


fn main() {
    println!("{}", unsafe { c_btoi(0) });
    println!("{}", unsafe { c_btoi(1) });
}
