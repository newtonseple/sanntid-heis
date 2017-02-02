use std::sync::mpsc;
use std::thread;

mod driver;
mod network;

mod que

pub enum PeerUpdate {
    NewPeer(usize),  // see ----------------------------------------------------\/
    LostPeer(usize),
}

pub enum RecievedMessage {
    OrderComplete{type: driver::elev_button_type_t, floor: i32},
    StateUpdate{direction: driver::elev_button_type_t, floor: i32},
    NewOrder{type: driver::elev_button_ type_t, floor: i32, id: usize}, // usize for use in array indexing, other types might be more appropriate
}

pub struct Order {
    floor: i32,
    type: driver::elev_button_type_t,
}

// how to handle local_cmd_request
pub fn start(hw_command_tx: mpsc::Sender<HwCommandMessage>,
             send_message_tx: mpsc::Sender<SendMessageCommand>,
             peer_update_rx: mpsc::Receiver<PeerUpdate>,
             add_order_rx: mpsc::Receiver<Order>
             message_recieved_rx: mpsc::Receiver<RecievedMessage>
             ) -> thread::JoinHandle<()> {
    
}