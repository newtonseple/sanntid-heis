/*
This module is responsible for reacting to outside requests for
new orders (buttons), delegating them, keeping track of all the
elevator states in a map of ElevatorData structs, as well as
distribution of possible lost orders when a lost peer reconnects.
*/

use std::sync::mpsc;
use std::thread;

use std::collections::HashMap;

use hardware_io;
use hardware_io::HwCommandMessage;
use hardware_io::OrderType;
use network;
use network::SendMessageCommand;
use local_controller;

mod queue;

pub use self::queue::ServiceDirection;

use self::queue::ElevatorData;

pub struct LocalCommandRequestMessage {
    pub floor: i32,
    pub current_service_direction: ServiceDirection,
}

#[derive(Copy, Clone)]
pub struct Order {
    pub floor: i32,
    pub order_type: OrderType,
}

pub fn run(hw_command_tx: mpsc::Sender<hardware_io::HwCommandMessage>,
           send_message_tx: mpsc::Sender<network::SendMessageCommand>,
           local_command_tx: mpsc::SyncSender<local_controller::LocalCommandMessage>,
           add_order_tx: mpsc::Sender<Order>,
           add_order_rx: mpsc::Receiver<Order>,
           peer_update_rx: mpsc::Receiver<network::PeerUpdate<String>>,
           message_recieved_rx: mpsc::Receiver<network::Packet<network::SendMessageCommand,
                                                               String>>,
           local_command_request_rx: mpsc::Receiver<LocalCommandRequestMessage>)
           -> thread::JoinHandle<()> {
    thread::Builder::new().name("planner".to_string()).spawn(move || {
        let mut elevator_data_map = HashMap::new();
        loop {
            select! {
                // the add_order channel contains orders for delegation,
                // and are handled here
                add_order_result = add_order_rx.recv() => {
                    let order = add_order_result.unwrap();
                    let local_id = network::get_localip().expect("could not get local ip");
                    let (best_id, _): (&String, &ElevatorData) = 
                        match order.order_type {
                            OrderType::CAB => (&local_id, elevator_data_map.get(&local_id).expect("could not get local lift data")),
                            _ => {
                                elevator_data_map.iter()
                                    .filter(|&(_, elevator_data): &(&String, &ElevatorData)| elevator_data.alive == true)
                                    .min_by_key(|&(_, elevator_data): &(&String, &ElevatorData)| {
                                        elevator_data.get_order_cost(order.floor, order.order_type)
                                    }).expect ("Could not find an elevator to handle the order")
                            }
                        };
                    send_message_tx.send(network::SendMessageCommand::NewOrder {
                        order_type: order.order_type,
                        floor: order.floor,
                        id: (*best_id).clone(),
                    }).expect("unable to send");
                },

                peer_update_result = peer_update_rx.recv() => {
                    let peer_update = peer_update_result.unwrap();
                    println!("{}", peer_update);

                    // If we are notified of a new/returning peer,
                    // give them the order table and make sure
                    // they have an entry in the map.
                    if let Some(id) = peer_update.new {
                        if elevator_data_map.contains_key(&id){
                            elevator_data_map.get_mut(&id).unwrap().alive = true;
                            println!("Peer returned from the dead: {}", id);
                            
                            // Distribution of existing order table
                            for (id, elevator_data) in elevator_data_map.iter(){
                                for order in elevator_data.get_orders(){
                                    send_message_tx.send(
                                        SendMessageCommand::NewOrder{
                                            order_type: order.order_type,
                                            floor: order.floor,
                                            id: id.clone(),
                                        }
                                    ).expect("could not send orders 989889");
                                }
                            }
                        } else {
                            elevator_data_map.insert(id.clone(), ElevatorData::new());
                            println!("Added new elevator_data for: {}", id);
                        }
                    }

                    for id in peer_update.lost {
                        elevator_data_map.get_mut(&id).unwrap().alive = false;
                        println!("Peer died: {}", id);

                        // The hall orders of the lost peer will be redelegated
                        for order in (*elevator_data_map.get(&id).unwrap()).get_orders().iter()
                            .filter(|&order| order.order_type != OrderType::CAB)
                        {
                            add_order_tx.send(*order).expect("Failed to queue order for new assignment");
                        }
                    }
                },

                // All network message handling happens here
                message_recieved_result = message_recieved_rx.recv() => {
                    let message_recieved = message_recieved_result.expect("message_recieved_result failed");
                    let local_ip = network::get_localip()
                        .expect("Could not get local ip");
                    match message_recieved.data {
                        network::SendMessageCommand::NewOrder{order_type, floor, id} => {

                            //Make sure the elevator exists in the map
                            if ! (elevator_data_map.contains_key(&id)){
                                elevator_data_map.insert(id.clone(), ElevatorData::new());
                                println!("Added new elevator_data for: {}", id);
                            }

                            elevator_data_map.get_mut(&id)
                                .expect(format!("ID not in map: {}",id).as_str())
                                .set_order(order_type, floor, true);
                            if id == local_ip || order_type == OrderType::UP || order_type == OrderType::DOWN {
                                hw_command_tx.send(HwCommandMessage::SetButtonLamp {
                                    button_type: order_type,
                                    floor: floor,
                                    value: true,
                                }).expect("Could not send light message");
                            }
                        },

                        network::SendMessageCommand::StateUpdate{direction, floor} => {
                            match elevator_data_map.get_mut(&message_recieved.id){
                                Some(elevator_data) => elevator_data.update_state(floor, direction),
                                None => {}
                            }
                        }

                        network::SendMessageCommand::OrderComplete{order_type, floor} => {
                            for (id, elevator_data) in elevator_data_map.iter_mut() {
                                elevator_data.set_order(order_type, floor, false);
                                if order_type == OrderType::UP || order_type == OrderType::DOWN {
                                    hw_command_tx.send(HwCommandMessage::SetButtonLamp{
                                        button_type: order_type,
                                        floor: floor,
                                        value: false,
                                    }).expect("Could not send light message");
                                }
                                if *id == message_recieved.id {
                                    elevator_data.set_order(OrderType::CAB, floor, false);
                                    if *id == local_ip {
                                        hw_command_tx.send(HwCommandMessage::SetButtonLamp{
                                            button_type: OrderType::CAB,
                                            floor: floor,
                                            value: false,
                                        }).expect("Could not send light message 3487");
                                    }
                                }
                            }
                        }
                    }
                },

                // This is the response to the local controller asking what to do,
                // provided by the ElevatorData representing the local elevator.
                local_command_request_result = local_command_request_rx.recv() => {
                    let local_command_request = local_command_request_result.expect("local_command_request_result failed");
                    let local_ip = network::get_localip()
                        .expect("Could not get local ip");
                    let local_elevator_data_option = elevator_data_map
                        .get_mut(&local_ip);
                    match local_elevator_data_option{
                        Some(local_elevator_data) => {
                            local_elevator_data.update_state(local_command_request.floor, local_command_request.current_service_direction);
                            let local_command = local_elevator_data.get_local_command();
                            local_command_tx.send(local_command)
                                .expect("Could not send local command");
                        }
                        None => {
                            local_command_tx.send(local_controller::LocalCommandMessage::DoNothing)
                                .expect("Could not send local command");
                        } 
                    }
                }
            }
        }
    }).expect("Failed to start thread")
}
