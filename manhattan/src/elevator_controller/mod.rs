use std::sync::mpsc;
use std::thread;

mod driver;

use driver::*;

pub enum hw_command {
    Set_button_light(floor, elev_button_type_t, state),  // Rename state to value?
    Set_door_lamp(state),
    And_so_on,
}

pub fn run(hw_command_rx: mpsc::Receiver<hw_command>, ... ) {
    thread::spawn(move || {
        loop {
            unimplemented!();
        }
    })
}