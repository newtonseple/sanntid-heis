use std::sync::mpsc;
use std::thread;
use std::net::IpAddr;

use std::cmp::Ordering;
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

pub fn start(hw_command_tx: mpsc::Sender<hardware_io::HwCommandMessage>,
             send_message_tx: mpsc::Sender<network::SendMessageCommand>,
             peer_update_rx: mpsc::Receiver<network::PeerUpdate<String>>,
             add_order_tx: mpsc::Sender<Order>,
             add_order_rx: mpsc::Receiver<Order>,
             message_recieved_rx: mpsc::Receiver<network::Packet<network::SendMessageCommand, String>>,
             local_command_request_rx: mpsc::Receiver<LocalCommandRequestMessage>,
             local_command_tx: mpsc::SyncSender<local_controller::LocalCommandMessage>)
             -> thread::JoinHandle<()> {
    thread::Builder::new().name("planner".to_string()).spawn(move || {
        let mut elevator_data_map = HashMap::new();
        loop {
            select! {
                add_order_result = add_order_rx.recv() => {
                    let order = add_order_result.unwrap();
                    let local_id = network::get_localip().expect("could not get local ip");
                    println!("got order button, {}. Delegating.", order.floor);
                    
                    let (best_id, _): (&String, &ElevatorData) = 
                        match order.order_type {
                            OrderType::CAB => (&local_id, elevator_data_map.get(&local_id).expect("could not get local lift data")),
                            _ => {
                                elevator_data_map.iter()
                                    .filter(|&(id, elevator_data): &(&String, &ElevatorData)| elevator_data.alive == true)
                                    .min_by_key(|&(id, elevator_data): &(&String, &ElevatorData)| {
                                        elevator_data.get_order_cost(order.floor, order.order_type)
                                    }).expect ("Could not find an elevator to handle the order")
                            }
                        };

                    
                    println!("\"Optimal\" elevator found: {}", best_id);

                    send_message_tx.send(network::SendMessageCommand::NewOrder {
                        order_type: order.order_type,
                        floor: order.floor,
                        id: (*best_id).clone(),
                    }).expect("unable to send 486983417965827346");
                },
                peer_update_result = peer_update_rx.recv() => {
                    let peer_update = peer_update_result.unwrap();
                    println!("{}", peer_update);
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

                        // Redelegation of orders belonging to dead peer
                        for order in (*elevator_data_map.get(&id).unwrap()).get_orders().iter()
                            .filter(|&order| order.order_type != OrderType::CAB)
                        {
                            add_order_tx.send(*order).expect("Failed to queue order for new assignment");
                        }

                    }
                    
                },
                message_recieved_result = message_recieved_rx.recv() => {
                    let message_recieved = message_recieved_result.expect("message_recieved_result failed");
                    println!("Got net message! {:?}", message_recieved);
                    let local_ip = network::get_localip()
                        .expect("Could not get local ip 99999783");
                    match message_recieved.data {
                        network::SendMessageCommand::NewOrder{order_type, floor, id} => {
                            println!("New order from network");
                            
                            //Make sure the entry exists in the order table
                            if ! (elevator_data_map.contains_key(&id)){
                                elevator_data_map.insert(id.clone(), ElevatorData::new());
                                println!("Added new elevator_data for: {}", id);
                            }

                            elevator_data_map.get_mut(&id)
                                .expect(format!("ID not in map: {}",id).as_str())
                                .set_order(order_type, floor, true);
                            if id == local_ip || order_type == OrderType::UP || order_type == OrderType::DOWN{
                                hw_command_tx.send(HwCommandMessage::SetButtonLamp{
                                    button_type: order_type,
                                    floor: floor,
                                    value: true,
                                }).expect("Could not send light message 3276487");
                            }
                        },
                        network::SendMessageCommand::StateUpdate{direction, floor} => {
                            println!("State upd rcv");
                            match elevator_data_map.get_mut(&message_recieved.id){
                                Some(elevator_data) => elevator_data.update_state(floor, direction),
                                None => println!("No elevator data for remote ip")
                            }
                        }
                        network::SendMessageCommand::OrderComplete{order_type, floor} => {
                            println!("Order complete");
                            for (id, elevator_data) in elevator_data_map.iter_mut() {
                                elevator_data.set_order(order_type, floor, false);
                                if order_type == OrderType::UP || order_type == OrderType::DOWN {
                                    hw_command_tx.send(HwCommandMessage::SetButtonLamp{
                                        button_type: order_type,
                                        floor: floor,
                                        value: false,
                                    }).expect("Could not send light message 32764");
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
                local_command_request_result = local_command_request_rx.recv() => {
                    //println!("got local command request");
                    let local_command_request = local_command_request_result.expect("local_command_request_result failed");
                    let local_ip = network::get_localip()
                        .expect("Could not get local ip 9999978");
                    let local_elevator_data_option = elevator_data_map
                        .get_mut(&local_ip);
                    match local_elevator_data_option{
                        Some(local_elevator_data) => {
                            local_elevator_data.update_state(local_command_request.floor, local_command_request.current_service_direction);
                            let local_command = local_elevator_data.get_local_command();
                            local_command_tx.send(local_command)
                                .expect("Could not send local command 66668234");
                        }
                        None => {
                            local_command_tx.send(local_controller::LocalCommandMessage::DoNothing)
                                .expect("Could not send local command 68234");
                        } 
                    }
                    //println!("sent local command");
                }
            }
        }
    }).expect("Failed to start thread")
}
