#![allow(dead_code)]
#![allow(unused_imports)]

extern crate manhattan;
use manhattan::*;

mod driver;

fn main() {
	driver::init();
	driver::go_up();
	driver::set_button_lamp(driver::elev_button_type_t::CAB, 0, 1);
	driver::set_button_lamp(driver::elev_button_type_t::CAB, 1, 0);
	driver::set_button_lamp(driver::elev_button_type_t::CAB, 2, 1);
	driver::set_button_lamp(driver::elev_button_type_t::CAB, 3, 0);
	driver::set_floor_indicator(2);
	println!("{}", driver::get_obstruction_signal());
	//driver::test_run();
}