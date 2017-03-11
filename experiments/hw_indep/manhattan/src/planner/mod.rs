use std::sync::mpsc;
use std::thread;
use std::net::IpAddr;

use std::collections::HashMap;

use hardware_io;
use hardware_io::OrderType;

use network;
use local_controller;


mod queue;

pub use self::queue::ServiceDirection;
use self::queue::ElevatorData;

pub struct LocalCommandRequestMessage {
    pub floor: i32,
    pub current_service_direction: ServiceDirection,
}

pub struct Order {
    pub floor: i32,
    pub order_type: OrderType,
}

pub fn start(hw_command_tx: mpsc::Sender<hardware_io::HwCommandMessage>,
             send_message_tx: mpsc::Sender<network::SendMessageCommand>,
             peer_update_rx: mpsc::Receiver<network::PeerUpdate<String>>,
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
                    println!("got order, {}. Delegating to self and telling network as test", order.floor);
                    //TODO: Delegation logic
                    send_message_tx.send(network::SendMessageCommand::NewOrder {
                        order_type: order.order_type,
                        floor: order.floor,
                        id: network::get_localip().expect("IP not got").to_string(),
                    }).expect("unable to send 486983417965827346");
                },
                peer_update_result = peer_update_rx.recv() => {
                    let peer_update = peer_update_result.unwrap();
                    println!("{}", peer_update);
                    if let Some(id) = peer_update.new {
                        elevator_data_map.insert(id, ElevatorData::new());
                        println!("{:?}", elevator_data_map);
                    }
                    
                },
                message_recieved_result = message_recieved_rx.recv() => {
                    let message_recieved = message_recieved_result.expect("message_recieved_result failed");
                    println!("Got net message! {:?}", message_recieved);
                    match message_recieved.data {
                        network::SendMessageCommand::NewOrder{order_type, floor, id} => {
                            println!("NEW ORDER NETWORK RECEIVED");
                            elevator_data_map.get_mut(&id)
                                .expect(format!("ID not in map: {}",id).as_str())
                                .set_order(order_type, floor, true);
                        },
                        network::SendMessageCommand::StateUpdate{direction, floor} => println!("State upd rcv"),
                        network::SendMessageCommand::OrderComplete{order_type, floor} => println!("Order complete"),
                        
                    }
                },
                local_command_request_result = local_command_request_rx.recv() => {
                    let local_command_request = local_command_request_result.expect("local_command_request_result failed");
                    let local_ip = network::get_localip()
                        .expect("Could not get local ip 9999978");
                    let local_elevator_data = elevator_data_map
                        .get_mut(&local_ip)
                        .expect("Could not get elevator data for local ip");
                    local_elevator_data.update_state(local_command_request.floor, local_command_request.current_service_direction);
                    let local_command = local_elevator_data.get_local_command();
                    local_command_tx.send(local_command)
                        .expect("Could not send local command 66668234");

                }
            }
        }
    }).expect("Failed to start thread")
}
