extern crate libc;

#[repr(C)]
pub enum c_bool_t {
    FALSE = 0,
    TRUE = 1,
}

extern "C" {
    pub fn c_btoi(c_bool: c_bool_t) -> i32;
}

fn main() {
    println!("{}", unsafe { c_btoi(c_bool_t::FALSE) });
    println!("{}", unsafe { c_btoi(c_bool_t::TRUE) });
}