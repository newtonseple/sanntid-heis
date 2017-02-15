use std::sync::mpsc;
use std::thread;

use hardware_io;
use network;

mod que;

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

pub struct Order {
    pub floor: i32,
    pub order_type: hardware_io::OrderType,
}

pub fn start(hw_command_tx: mpsc::Sender<hardware_io::HwCommandMessage>,
             send_message_tx: mpsc::Sender<network::SendMessageCommand>,
             peer_update_rx: mpsc::Receiver<PeerUpdate>,
             add_order_rx: mpsc::Receiver<Order>,
             message_recieved_rx: mpsc::Receiver<RecievedMessage>)
             -> thread::JoinHandle<()> {
                unimplemented!();

}
