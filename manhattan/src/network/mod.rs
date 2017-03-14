use std::sync::mpsc;
use std::thread;

pub mod localip;
mod bcast;
mod peer;

pub use self::peer::PeerUpdate;
pub use self::bcast::SendMessageCommand;
pub use self::bcast::Packet;
pub use self::localip::get_localip;

use self::peer::*;
use self::bcast::*;

const PEER_PORT: u16 = 5177;
const BCAST_PORT: u16 = 5176;

pub fn start(send_message_rx: mpsc::Receiver<SendMessageCommand>,
             i_am_stuck_rx: mpsc::Receiver<()>,
             message_recieved_tx: mpsc::Sender<Packet<SendMessageCommand, String>>,
             peer_update_tx: mpsc::Sender<PeerUpdate<String>>) {
    // Creates network thread
    // Creates PeerTransmitter thread
    thread::spawn(move || {
        let id = format!("{}", get_localip().expect("failed to get local ip"));
        //let id = get_localip().expect("failed to get local ip");
        PeerTransmitter::new(PEER_PORT).expect("Error creating PeerTransmitter").run(i_am_stuck_rx,
                                                                                     &id);
    });

    // Creates PeerReciever thread
    thread::spawn(move || {
                      PeerReceiver::new(PEER_PORT)
                          .expect("Error creating PeerReceiver")
                          .run(peer_update_tx);
                  });

    let message_recieved_tx_loopback = message_recieved_tx.clone();

    // Creates BcastTransmitter thread
    thread::spawn(move || {
                      BcastTransmitter::new(BCAST_PORT)
                          .expect("Error creating BcastTransmitter")
                          .run(send_message_rx, message_recieved_tx_loopback);

                  });

    // Creates BcastReciever thread
    thread::spawn(move || {
                      BcastReceiver::new(BCAST_PORT)
                          .expect("Error creating BcastReciever")
                          .run(message_recieved_tx);
                  });
}
