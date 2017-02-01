#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//#![allow(non_camel_case_types)]
#![allow(safe_extern_statics)]
#![feature(mpsc_select)]


use std::thread;
use std::sync::mpsc;

mod hardware_io;

fn main() {
    println!("Running");
    let (hw_command_tx, hw_command_rx): (mpsc::Sender<hardware_io::HwCommandMessage>,
                                         mpsc::Receiver<hardware_io::HwCommandMessage>) =
        mpsc::channel();
    let hardware_io_thread = hardware_io::run(hw_command_rx);

    let msg_test: hardware_io::HwCommandMessage =
        hardware_io::HwCommandMessage::SetDoorOpenLamp { value: true };

    let msg_test_motor: hardware_io::HwCommandMessage =
        hardware_io::HwCommandMessage::SetMotorDirection {
            direction: hardware_io::elev_motor_direction_t::UP,
        };

    hw_command_tx.send(msg_test_motor).unwrap();

    hardware_io_thread.join().unwrap();
    panic!("Exited the main thread!?");
}