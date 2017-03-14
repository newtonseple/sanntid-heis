use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use hardware_io;
use hardware_io::{HwCommandMessage, MotorDirection};
use network;
use network::SendMessageCommand;
use planner;
use planner::ServiceDirection;

pub enum LocalEventMessage {
    TimerTick,
    ArrivedAtFloor { floor: i32 },
}

pub enum LocalCommandMessage {
    GoUp,
    GoDown,
    DoNothing,
    StopForOrder { order_type: hardware_io::OrderType },
}

const TIME_BETWEEN_FLOORS: i32 = 5 * 10;
const TIME_DOOR_OPEN: i32 = 3 * 10;

pub fn run(hw_command_tx: mpsc::Sender<HwCommandMessage>,
           send_message_tx: mpsc::Sender<network::SendMessageCommand>,
           local_command_request_tx: mpsc::SyncSender<planner::LocalCommandRequestMessage>,
           i_am_stuck_tx: mpsc::Sender<()>,
           local_command_rx: mpsc::Receiver<LocalCommandMessage>,
           local_event_rx: mpsc::Receiver<LocalEventMessage>)
           -> thread::JoinHandle<()> {
    thread::Builder::new().name("local_controller".to_string()).spawn(move || {
        let mut timer = 0;
        let mut service_direction = ServiceDirection::IDLE;
        let mut servicing_order = false;
        let mut floor = 0;

        // Initializing. Getting the elevator to a known state
        println!("Starting init");
        loop {
            match local_event_rx.try_recv() {
                Ok(LocalEventMessage::ArrivedAtFloor{floor: 0}) => {
                    hw_command_tx.send(HwCommandMessage::SetMotorDirection{direction: MotorDirection::STOP}).expect("could not send HW command 324798");
                    break;
                },
                _ => {
                    hw_command_tx.send(HwCommandMessage::SetMotorDirection{direction: MotorDirection::DOWN})
                        .expect("could not send HW command 89074350832459876");
                    sleep(Duration::from_millis(20));
                    continue;
                },
            }
        }
        println!("Completed Init");

        loop {
            match local_event_rx.recv().expect("Unable to recieve local event") {
                LocalEventMessage::TimerTick => {
                    if timer > 0 {
                        timer -= 1;
                    }
                    match servicing_order {
                        true => {
                            if timer <=0 {
                                request_and_execute_local_command(&local_command_request_tx, &hw_command_tx, &send_message_tx, &local_command_rx, &floor, &mut servicing_order, &mut service_direction, &mut timer);
                            }
                        }
                        false => {
                            match service_direction{
                                ServiceDirection::UP | ServiceDirection::DOWN if timer <= 0 => {
                                    //We have been travelling for too long. Assume dead.
                                    //println!("Sending i_am_stuck as a test");
                                    i_am_stuck_tx.send(()).expect("Error sending i_am_stuck");
                                },
                                ServiceDirection::IDLE => {
                                    request_and_execute_local_command(&local_command_request_tx, &hw_command_tx, &send_message_tx, &local_command_rx, &floor, &mut servicing_order, &mut service_direction, &mut timer);
                                },
                                _ => continue,
                            }
                        }
                    }
                },
                LocalEventMessage::ArrivedAtFloor{floor: new_floor} => {
                    floor = new_floor;
                    hw_command_tx.send(HwCommandMessage::SetFloorIndicator{
                        floor: floor,
                    }).expect("Could not send floor indicator message 3276487");
                    //println!("local_controller got arrived, {}",floor);
                    request_and_execute_local_command(&local_command_request_tx, &hw_command_tx, &send_message_tx, &local_command_rx, &floor, &mut servicing_order, &mut service_direction, &mut timer);              },
            }
        }
    }).expect("Failed to start thread")
}

fn request_and_execute_local_command(local_command_request_tx: &mpsc::SyncSender<planner::LocalCommandRequestMessage>,
    hw_command_tx: &mpsc::Sender<HwCommandMessage>,
    send_message_tx: &mpsc::Sender<network::SendMessageCommand>,
    local_command_rx: &mpsc::Receiver<LocalCommandMessage>,
    floor: &i32,
    servicing_order: &mut bool,
    service_direction: &mut ServiceDirection,
timer: &mut i32){
    let previous_service_direction = *service_direction;
    //println!("Starting req'n'execute local command");
    local_command_request_tx.send(planner::LocalCommandRequestMessage {
                                      floor: *floor,
                                      current_service_direction: *service_direction,
                                  })
        .expect("Unable to send local_command_request");
    match local_command_rx.recv().expect("Unable to recieve local command") {
        LocalCommandMessage::DoNothing => {
            *service_direction = ServiceDirection::IDLE;
            hw_command_tx.send(HwCommandMessage::SetMotorDirection {
                                   direction: MotorDirection::STOP,
                               })
                .expect("Unable to send HwCommandMessage 677777293487");
            *servicing_order = false;
            hw_command_tx.send(HwCommandMessage::SetDoorOpenLamp{value: false})
                .expect("Unable to send hw_command");
        }
        LocalCommandMessage::GoDown => {
            hw_command_tx.send(HwCommandMessage::SetDoorOpenLamp{value: false})
                .expect("Unable to send hw_command");
            *servicing_order = false;
            hw_command_tx.send(HwCommandMessage::SetMotorDirection {
                                   direction: MotorDirection::DOWN,
                               })
                .expect("Unable to send hw_command");
            *service_direction = ServiceDirection::DOWN;
            *timer = TIME_BETWEEN_FLOORS;
        }
        LocalCommandMessage::GoUp => {
            hw_command_tx.send(HwCommandMessage::SetDoorOpenLamp{value: false})
                .expect("Unable to send hw_command");
            *servicing_order = false;
            hw_command_tx.send(HwCommandMessage::SetMotorDirection {
                                   direction: MotorDirection::UP,
                               })
                .expect("Unable to send hw_command");
            *service_direction = ServiceDirection::UP;
            *timer = TIME_BETWEEN_FLOORS;
        }
        LocalCommandMessage::StopForOrder { order_type } => {
            *service_direction = match order_type {
                hardware_io::OrderType::UP => ServiceDirection::UP,
                hardware_io::OrderType::DOWN => ServiceDirection::DOWN,
                _ => *service_direction,
            };
            hw_command_tx.send(HwCommandMessage::SetMotorDirection {
                                   direction: MotorDirection::STOP,
                               })
                .expect("Unable to stop motor");
            *servicing_order = true;
            *timer = TIME_DOOR_OPEN;
            hw_command_tx.send(HwCommandMessage::SetDoorOpenLamp{value: true}).expect("Unable to open door");
            send_message_tx.send(SendMessageCommand::OrderComplete {
                                     order_type: order_type,
                                     floor: *floor,
                                 })
                .expect("Failed to send order complete 23423476");
        }
    }

    //If we are not just standing still, tell everyone what just happened.
    if !(*service_direction == ServiceDirection::IDLE &&
         previous_service_direction == ServiceDirection::IDLE) {
        send_message_tx.send(SendMessageCommand::StateUpdate {
                                 direction: *service_direction,
                                 floor: *floor,
                             })
            .expect("Unable to send message 8749385333333333345");
    }

    //println!("Done with req'n'execute local command");
}
