mod c_driver;


pub use self::c_driver::N_FLOORS;

pub use self::c_driver::OrderType;
pub use self::c_driver::MotorDirection;

//pub type OrderType = elev_button_type_t;

//pub type MotorDirection = elev_motor_direction_t;

//TODO: change ifs to asserts

pub fn init() {
    unsafe {
        c_driver::elev_init();
    }
}

pub fn set_motor_direction(direction: MotorDirection) {
    unsafe {
        c_driver::elev_set_motor_direction(direction);
    }
}

pub fn set_button_lamp(button: OrderType, floor: i32, value: bool) {
    if floor >= 0 && floor < N_FLOORS as i32 {
        unsafe {
            c_driver::elev_set_button_lamp(button, floor, value as i32);
        }
    } else {
        panic!("Tried to set button lamp in nonexisting floor!")
    }
}

pub fn set_floor_indicator(floor: i32) {
    assert!(floor >= 0 && floor < N_FLOORS as i32,
            "Tried to set the floor in {}th floor (floor not existing)",
            floor);
    unsafe {
        c_driver::elev_set_floor_indicator(floor);
    }
}

pub fn set_door_open_lamp(value: bool) {
    unsafe {
        c_driver::elev_set_door_open_lamp(value as i32);
    }
}

pub fn set_stop_lamp(value: bool) {
    unsafe {
        c_driver::elev_set_stop_lamp(value as i32);
    }
}


pub fn get_button_signal(button: OrderType, floor: i32) -> bool {
    if floor >= 0 && floor < N_FLOORS as i32 {
        unsafe { c_driver::elev_get_button_signal(button, floor) != 0 }
    } else {
        panic!("Tried to get a button signal in a nonexisting floor, {}",
               floor)
    }
}

// returns the floor (0-indexed) or None
pub fn get_floor_sensor_signal() -> Option<i32> {
    let result = unsafe { c_driver::elev_get_floor_sensor_signal() };
    if result != -1 { Some(result) } else { None }
}

pub fn get_stop_signal() -> bool {
    unsafe { c_driver::elev_get_stop_signal() != 0 }
}

pub fn get_obstruction_signal() -> bool {
    unsafe { c_driver::elev_get_obstruction_signal() != 0 }
}


pub fn test_run() -> ! {
    unsafe {
        c_driver::test_run();
    }
    loop {}
}
