mod c_driver;
pub use self::c_driver::elev_button_type_t;

pub fn go_up() {
	println!("going up");
}
pub fn test_run() -> ! {
	unsafe { c_driver::test_run(); }
	loop {};
}
pub fn set_button_lamp(button: elev_button_type_t , floor: i32, value: i32){
	unsafe { c_driver::elev_set_button_lamp(button, floor, value); }
}

pub fn set_floor_indicator(floor: i32) {
	unsafe { c_driver::elev_set_floor_indicator(floor); }
}

pub fn init() {
	unsafe { c_driver::elev_init(); }
}

pub fn get_obstruction_signal() -> bool {
	unsafe { c_driver::elev_get_obstruction_signal() != 0 }
}