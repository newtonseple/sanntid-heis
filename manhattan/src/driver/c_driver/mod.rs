extern crate libc;



//#[link(name = "driver", kind = "static")]
extern "C" {
	pub fn elev_init();
    pub fn test_run() -> i32;
    pub fn elev_set_floor_indicator(floor: i32);
    pub fn elev_set_button_lamp(button: elev_button_type_t , floor: i32, value: i32);
    pub fn elev_get_obstruction_signal() -> i32;
}

#[repr(C)]
pub enum elev_button_type_t { 
    UP = 0,
    DOWN = 1,
    CAB = 2,
}