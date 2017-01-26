mod c_driver;

pub use self::c_driver::elev_button_type_t;
pub use self::c_driver::elev_motor_direction_t;
pub use self::c_driver::n_floors; // Is this needed? As pub?

pub fn init() {
    unsafe { c_driver::elev_init(); }
}

pub fn set_motor_direction(direction: elev_motor_direction_t) {
    unsafe { c_driver::elev_set_motor_direction(direction); }
}

pub fn set_button_lamp(button: elev_button_type_t, floor: i32, value: bool) {
    match floor {
		0..n_floors => unsafe { c_driver::elev_set_button_lamp(button, floor, value as i32); },
		_ => panic!("Tried to set button lamp in non-existing floor!");
	}    
}

pub fn set_floor_indicator(floor: i32) {
	match floor {
		0..n_floors => unsafe { c_driver::elev_set_floor_indicator(floor); },
		_ => panic!("Tried to set the floor indicator in a nonexisting floor!");
	}
    
}

pub fn set_door_open_lamp(value: bool) {
    unsafe { c_driver::elev_set_door_open_lamp(value as i32); }
}

pub fn set_stop_lamp(value: bool) {
    unsafe { c_driver::elev_set_stop_lamp(value as i32); }
}


pub fn get_button_signal(button: elev_button_type_t, floor: i32) -> bool {
	match floor {
		0..n_floors => unsafe { c_driver::elev_get_button_signal(button, floor) != 0 },
		_ => panic!("Tried to get a button signal in a nonexisting floor");
	}
    
}

// returns the floor (0-indexed) or -1 if the carriage is between floors
pub fn get_floor_sensor_signal() -> i32 {
    unsafe { c_driver::elev_get_floor_sensor_signal() }
}

pub fn get_stop_signal() -> bool {
    unsafe { c_driver::elev_get_stop_signal() != 0 }
}

pub fn get_obstruction_signal() -> bool {
    unsafe { c_driver::elev_get_obstruction_signal() != 0 }
}


pub fn test_run() -> ! {
    unsafe { c_driver::test_run(); }
    loop {}
}