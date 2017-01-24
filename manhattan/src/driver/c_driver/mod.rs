extern crate libc;



//#[link(name = "driver", kind = "static")]
extern "C" {
    pub fn elev_init();

    pub fn elev_set_motor_direction(dirn: elev_motor_direction_t);
    pub fn elev_set_button_lamp(button: elev_button_type_t, floor: i32, value: i32);
    pub fn elev_set_floor_indicator(floor: i32);
    pub fn elev_set_door_open_lamp(value: i32);
    pub fn elev_set_stop_lamp(value: i32);

    pub fn elev_get_button_signal(button: elev_button_type_t, floor: i32) -> i32;
    pub fn elev_get_floor_sensor_signal() -> i32;
    pub fn elev_get_stop_signal() -> i32;
    pub fn elev_get_obstruction_signal() -> i32;

    pub fn test_run() -> i32;
}

#[repr(C)]
pub enum elev_button_type_t {
    UP = 0,
    DOWN = 1,
    CAB = 2,
}

#[repr(C)]
pub enum elev_motor_direction_t {
    DOWN = -1,
    STOP = 0,
    UP = 1,
}