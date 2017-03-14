extern crate libc;

#[allow(dead_code)] // This is a general module, so it is fine if we don't use all of it.
extern "C" {
    pub fn elev_init();

    pub fn elev_set_motor_direction(dirn: MotorDirection);
    pub fn elev_set_button_lamp(button: OrderType, floor: libc::c_int, value: libc::c_int);
    pub fn elev_set_floor_indicator(floor: libc::c_int);
    pub fn elev_set_door_open_lamp(value: libc::c_int);
    pub fn elev_set_stop_lamp(value: libc::c_int);

    pub fn elev_get_button_signal(button: OrderType, floor: libc::c_int) -> libc::c_int;
    pub fn elev_get_floor_sensor_signal() -> libc::c_int;
    pub fn elev_get_stop_signal() -> libc::c_int;
    pub fn elev_get_obstruction_signal() -> libc::c_int;
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(C)]
pub enum OrderType {
    UP = 0,
    DOWN = 1,
    CAB = 2,
}

#[repr(C)]
pub enum MotorDirection {
    DOWN = -1,
    STOP = 0,
    UP = 1,
}

// This should be the same as N_FLOORS defined in elev.h
pub const N_FLOORS: i32 = 4;
