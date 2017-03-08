// External dependencies
use std::sync::mpsc;
use std::thread;
use std::env::args;

use planner;
use local_controller;

// Internal dependencies
mod rust_driver;
use self::rust_driver::*;

pub use self::rust_driver::OrderType;
pub use self::rust_driver::MotorDirection;
pub use self::rust_driver::N_FLOORS;


pub enum HwCommandMessage {
    SetButtonLamp {
        button_type: OrderType,
        floor: i32,
        value: bool,
    },
    SetDoorOpenLamp { value: bool },
    SetMotorDirection { direction: MotorDirection },
}


pub fn start(local_event_tx: mpsc::Sender<local_controller::LocalEventMessage>,
             add_order_tx: mpsc::Sender<planner::Order>,
             hw_command_rx: mpsc::Receiver<HwCommandMessage>)
             -> thread::JoinHandle<()> {
    thread::Builder::new()
        .name("hardware_io".to_string())
        .spawn(move || {
            init(); //Hardware initialization

            //states for edge detection
            let mut button_already_pressed = [[false; N_FLOORS as usize]; 3];
            let mut floor_already_reached = -1;
            loop {
                if let Ok(command) = hw_command_rx.try_recv() {
                    println!("Got hw_command");
                    match command {
                        HwCommandMessage::SetButtonLamp { button_type, floor, value } => {
                            set_button_lamp(button_type, floor, value)
                        }
                        HwCommandMessage::SetDoorOpenLamp { value } => set_door_open_lamp(value),
                        HwCommandMessage::SetMotorDirection { direction } => {
                            set_motor_direction(direction)
                        }
                    }
                }

                // Poll all hardware inputs

                if let Some(floor_sensor_result) = get_floor_sensor_signal() {
                    if floor_sensor_result != floor_already_reached {
                        local_event_tx.send(local_controller::LocalEventMessage::ArrivedAtFloor {
                                floor: floor_sensor_result,
                            })
                            .unwrap();
                    }
                    floor_already_reached = floor_sensor_result;
                }

                for floor in 0..N_FLOORS {
                    if get_button_signal(OrderType::UP, floor) {
                        if !button_already_pressed[OrderType::UP as usize][floor as usize] {
                            add_order_tx.send(planner::Order {
                                    floor: floor,
                                    order_type: OrderType::UP,
                                })
                                .unwrap();
                        }
                        button_already_pressed[OrderType::UP as usize][floor as usize] = true;
                    } else {
                        button_already_pressed[OrderType::UP as usize][floor as usize] = false;
                    }

                }

                for floor in 0..N_FLOORS {

                    if get_button_signal(OrderType::DOWN, floor) {
                        if !button_already_pressed[OrderType::DOWN as usize][floor as usize] {
                            add_order_tx.send(planner::Order {
                                    floor: floor,
                                    order_type: OrderType::DOWN,
                                })
                                .unwrap();
                        }
                        button_already_pressed[OrderType::DOWN as usize][floor as usize] = true;
                    } else {
                        button_already_pressed[OrderType::DOWN as usize][floor as usize] = false;
                    }

                }

                for floor in 0..N_FLOORS {
                    if get_button_signal(OrderType::CAB, floor) {
                        if !button_already_pressed[OrderType::CAB as usize][floor as usize] {
                            add_order_tx.send(planner::Order {
                                    floor: floor,
                                    order_type: OrderType::CAB,
                                })
                                .unwrap();
                        }
                        button_already_pressed[OrderType::CAB as usize][floor as usize] = true;
                    } else {
                        button_already_pressed[OrderType::CAB as usize][floor as usize] = false;
                    }

                }
                if get_stop_signal() {
                    set_motor_direction(MotorDirection::STOP);
                    panic!("STOP!!!"); //TODO MAYBE: Slightly more graceful exit...
                }

                // Input with unspesified behaviour
            /*
            if get_obstruction_signal() {
                unimplemented!();
            }
            */







            }
        })
        .expect("Failed to start thread")
}
