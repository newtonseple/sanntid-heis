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

use std::sync::mpsc;

mod hardware_io;
mod timer;
mod planner;
mod local_controller;
mod network;


fn main() {
    println!("Elevator node starting...");

    let (local_event_tx, local_event_rx) = mpsc::channel();
    let (add_order_tx, add_order_rx) = mpsc::channel();
    let (hw_command_tx, hw_command_rx) = mpsc::channel();
    let (send_message_tx, send_message_rx) = mpsc::channel();
    let (message_recieved_tx, message_recieved_rx) = mpsc::channel();
    let (peer_update_tx, peer_update_rx) = mpsc::channel();
    let (i_am_stuck_tx, i_am_stuck_rx) = mpsc::channel();
    let (local_command_tx, local_command_rx) = mpsc::sync_channel(0);
    let (local_command_request_tx, local_command_request_rx) = mpsc::sync_channel(0);

    let hardware_io_thread =
        hardware_io::run(local_event_tx.clone(), add_order_tx.clone(), hw_command_rx);
    let timer_thread = timer::run(local_event_tx.clone());
    let planner_thread = planner::run(hw_command_tx.clone(),
                                      send_message_tx.clone(),
                                      local_command_tx,
                                      add_order_tx.clone(),
                                      add_order_rx,
                                      peer_update_rx,
                                      message_recieved_rx,
                                      local_command_request_rx);
    let local_controller_thread = local_controller::run(hw_command_tx.clone(),
                                                        send_message_tx.clone(),
                                                        local_command_request_tx,
                                                        i_am_stuck_tx,
                                                        local_command_rx,
                                                        local_event_rx);

    network::start(send_message_rx,
                   i_am_stuck_rx,
                   message_recieved_tx,
                   peer_update_tx);

    // If the threads exit, a grave error has occured.
    hardware_io_thread.join().unwrap();
    timer_thread.join().unwrap();
    planner_thread.join().unwrap();
    local_controller_thread.join().unwrap();
    unreachable!();
}
