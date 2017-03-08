use std::sync::mpsc;
use std::thread;
use std::net::IpAddr;
use hardware_io;
use network;
use local_controller;

mod queue;

pub use self::queue::ServiceDirection;

pub struct LocalCommandRequestMessage {
    pub floor: i32,
    pub current_service_direction: ServiceDirection,
}

pub struct Order {
    pub floor: i32,
    pub order_type: hardware_io::OrderType,
}

pub fn start(hw_command_tx: mpsc::Sender<hardware_io::HwCommandMessage>,
             send_message_tx: mpsc::Sender<network::SendMessageCommand>,
             peer_update_rx: mpsc::Receiver<network::PeerUpdate<String>>,
             add_order_rx: mpsc::Receiver<Order>,
             message_recieved_rx: mpsc::Receiver<network::Packet<network::SendMessageCommand, String>>,
             local_command_request_rx: mpsc::Receiver<LocalCommandRequestMessage>,
             local_command_tx: mpsc::SyncSender<local_controller::LocalCommandMessage>)
             -> thread::JoinHandle<()> {
    thread::Builder::new().name("planner".to_string()).spawn(move || loop {
        select! {
            add_order_result = add_order_rx.recv() => {
                let order = add_order_result.unwrap();
                println!("got order, {}. Delegating to self and telling network as test", order.floor);
                send_message_tx.send(network::SendMessageCommand::NewOrder {
                    order_type: order.order_type,
                    floor: order.floor,
                    id: network::get_localip().expect("This is not happening").to_string(),
                }).expect("unable to send 486983417965827346");
            },
            peer_update_result = peer_update_rx.recv() => {
                let peer_update = peer_update_result.unwrap();
                println!("{}", peer_update)
            },
            message_recieved_result = message_recieved_rx.recv() => {
                let message_recieved = message_recieved_result.expect("message_recieved_result failed");
                println!("{:?}", message_recieved)
            }
        }
    }).expect("Failed to start thread")
}
