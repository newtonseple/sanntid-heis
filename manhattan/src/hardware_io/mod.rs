use std::sync::mpsc;
use std::thread;

mod rust_driver;

use self::rust_driver::*;
pub use self::rust_driver::elev_button_type_t;
pub use self::rust_driver::elev_motor_direction_t;

pub enum HwCommandMessage {
    SetButtonLamp {
        button_type: elev_button_type_t,
        floor: i32,
        value: bool,
    },
    SetDoorOpenLamp { value: bool },
    SetMotorDirection { direction: elev_motor_direction_t },
}

pub fn run(hw_command_rx: mpsc::Receiver<HwCommandMessage>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        init();
        loop{
            select! {
                command_result = hw_command_rx.recv() => {
                    match command_result.unwrap() {
                        HwCommandMessage::SetButtonLamp{button_type, floor, value} => set_button_lamp(button_type, floor, value),
                        HwCommandMessage::SetDoorOpenLamp{value} => set_door_open_lamp(value),
                        HwCommandMessage::SetMotorDirection{direction} => set_motor_direction(direction)
                    }
                }
            }
        }
    })
}