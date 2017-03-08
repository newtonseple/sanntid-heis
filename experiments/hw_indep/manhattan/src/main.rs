#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//#![allow(non_camel_case_types)]
#![allow(safe_extern_statics)]
#![feature(mpsc_select)]

#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate net2;
extern crate rand;
extern crate chrono;

use std::thread;
use std::sync::mpsc;

mod hardware_io;
mod planner;
mod local_controller;
mod network;


fn main() {
    println!("Starting modules...");
    let (local_event_tx, local_event_rx) = mpsc::channel();
    let (add_order_tx, add_order_rx) = mpsc::channel();
    let (hw_command_tx, hw_command_rx) = mpsc::channel();
    let (send_message_tx, send_message_rx) = mpsc::channel();
    let (message_recieved_tx, message_recieved_rx) = mpsc::channel();
    let (peer_update_tx, peer_update_rx) = mpsc::channel();
    let (i_am_stuck_tx, i_am_stuck_rx) = mpsc::channel();
    let (local_command_tx, local_command_rx) = mpsc::sync_channel(0);
    let (local_command_request_tx, local_command_request_rx) = mpsc::sync_channel(0);

    let hardware_io_thread = hardware_io::start(local_event_tx, add_order_tx, hw_command_rx); //DRIVER IX COMPLETE
    let planner_thread = planner::start(hw_command_tx.clone(), //PLANNER IX COMPLETE
                                        send_message_tx.clone(),
                                        peer_update_rx,
                                        add_order_rx,
                                        message_recieved_rx,
                                        local_command_request_rx,
                                        local_command_tx);
    let local_controller_thread = local_controller::start(local_event_rx, hw_command_tx.clone(), send_message_tx.clone(), local_command_request_tx, i_am_stuck_tx, local_command_rx); //LOCAL CTRL IX COMPLETE

    let network_thread = network::start(send_message_rx, i_am_stuck_rx, message_recieved_tx, peer_update_tx);


    let msg_test: hardware_io::HwCommandMessage =
        hardware_io::HwCommandMessage::SetDoorOpenLamp { value: true };

    let msg_test_motor: hardware_io::HwCommandMessage =
        hardware_io::HwCommandMessage::SetMotorDirection {
            direction: hardware_io::MotorDirection::UP,
        };

    //hw_command_tx.send(msg_test_motor).unwrap();

    hardware_io_thread.join().unwrap();
    planner_thread.join().unwrap();
    local_controller_thread.join().unwrap();
    panic!("Exited the main thread!?");
}
