use std::sync::mpsc;
use std::thread;

use hardware_io;
use network;
use local_controller;

mod que;

pub use self::que::ServiceDirection;

pub enum PeerUpdate {
    NewPeer(i32), // see ----------------------------------------------------\/
    LostPeer(i32),
}

pub enum RecievedMessage {
    OrderComplete {
        order_type: hardware_io::OrderType,
        floor: i32,
    },
    StateUpdate {
        direction: hardware_io::OrderType,
        floor: i32,
    },
    NewOrder {
        order_type: hardware_io::OrderType,
        floor: i32,
        id: i32,
    }, // usize for use in array indexing, other types might be more appropriate
}

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
             peer_update_rx: mpsc::Receiver<PeerUpdate>,
             add_order_rx: mpsc::Receiver<Order>,
             message_recieved_rx: mpsc::Receiver<RecievedMessage>,
             local_command_request_rx: mpsc::Receiver<LocalCommandRequestMessage>,
             local_command_tx: mpsc::SyncSender<local_controller::LocalCommandMessage>)
             -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        select! {
            add_order_result = add_order_rx.recv() => {
                let order = add_order_result.unwrap();
                println!("got order, {}", order.floor);
            }
        }
    })
}
