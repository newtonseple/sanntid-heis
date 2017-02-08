// External dependencies
use std::sync::mpsc;
use std::thread;

//mod ROOT.local_controller;
//use super::local_controller;
//mod planner;
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
    thread::spawn(move || {
        init();
        loop {
            if let Ok(command) = hw_command_rx.try_recv() {
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

            let floor_sensor_result = get_floor_sensor_signal();
            if floor_sensor_result != -1 {
                local_event_tx.send(local_controller::LocalEventMessage::ArrivedAtFloor {
                    floor: floor_sensor_result,
                });
            }

            for floor in 0..N_FLOORS {
                if get_button_signal(OrderType::UP, floor) {
                    add_order_tx.send(planner::Order {
                            Floor: floor,
                            OrderType: OrderType::UP,
                        })
                        .unwrap();
                }
            }

            for floor in 0..N_FLOORS {
                if get_button_signal(OrderType::DOWN, floor) {
                    add_order_tx.send(planner::Order {
                            Floor: floor,
                            OrderType: OrderType::DOWN,
                        })
                        .unwrap();
                }
            }

            for floor in 0..N_FLOORS {
                if get_button_signal(OrderType::CAB, floor) {
                    add_order_tx.send(planner::Order {
                            Floor: floor,
                            OrderType: OrderType::CAB,
                        })
                        .unwrap();
                }
            }

            // Input with unspesified behaviour
            /*
            if get_stop_signal() {
                unimplemented!();
            }

            if get_obstruction_signal() {
                unimplemented!();
            }
            */


















        }

    })
}
