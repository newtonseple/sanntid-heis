// External dependencies
use std::sync::mpsc;
use std::thread;

mod local_controller;
mod planner;

// Internal dependencies
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


pub fn start(local_event_tx: mpsc::Sender<local_corntroller::LocalEventMessage>, 
             add_order_tx: mpsc::channel<planner::Order>, 
             hw_command_rx: mpsc::Receiver<HwCommandMessage>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        init();
        loop {
            if let Ok(command) = hw_command_rx.try_recv() {
                match command {
                    HwCommandMessage::SetButtonLamp { button_type, floor, value } => set_button_lamp(button_type, floor, value),
                    HwCommandMessage::SetDoorOpenLamp { value } => set_door_open_lamp(value),
                    HwCommandMessage::SetMotorDirection { direction } => set_motor_direction(direction),
                }
            }
            
            // Poll all hardware inputs

            let floor_sensor_result = get_floor_sensor_signal();
            if floor_sensor_result != -1 {
                local_event_tx.send(local_controller::LocalEventMessage::ArrivedAtFloor{ floor = floor_sensor_result };
            }

            for floor in 0..N_FLOORS {
                if get_button_signal(elev_button_type_t::UP, floor) {
                    add_order_tx.send({floor: floor, type: elev_button_type_t::Up}).unwrap();
                }
            }

            for floor in 0..N_FLOORS {
                if get_button_signal(elev_button_type_t::DOWN, floor) {
                    add_order_tx.send({floor: floor, type: elev_button_type_t::DOWN}).unwrap();
                }
            }

            for floor in 0..N_FLOORS {
                if get_button_signal(elev_button_type_t::CAB, floor) {
                    add_order_tx.send({floor: floor, type: elev_button_type_t::CAB}).unwrap();
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
        }

    })
}