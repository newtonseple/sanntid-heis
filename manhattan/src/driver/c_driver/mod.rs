extern crate libc;



//#[link(name = "driver", kind = "static")]
extern "C" {
    pub static N_FLOORS: libc::c_int;

    pub fn elev_init();

    pub fn elev_set_motor_direction(dirn: elev_motor_direction_t);
    pub fn elev_set_button_lamp(button: elev_button_type_t,
                                floor: libc::c_int,
                                value: libc::c_int);
    pub fn elev_set_floor_indicator(floor: libc::c_int);
    pub fn elev_set_door_open_lamp(value: libc::c_int);
    pub fn elev_set_stop_lamp(value: libc::c_int);

    pub fn elev_get_button_signal(button: elev_button_type_t, floor: libc::c_int) -> libc::c_int;
    pub fn elev_get_floor_sensor_signal() -> libc::c_int;
    pub fn elev_get_stop_signal() -> libc::c_int;
    pub fn elev_get_obstruction_signal() -> libc::c_int;

    pub fn test_run() -> libc::c_int;
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