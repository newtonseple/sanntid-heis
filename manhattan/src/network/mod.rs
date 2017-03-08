use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;
use std::sync::mpsc::channel;
use std::net;

use rand;
use rand::Rng;
use chrono::offset::local::Local;

pub mod localip;
mod bcast;
mod peer;

pub use self::peer::PeerUpdate;
pub use self::bcast::SendMessageCommand;
pub use self::bcast::Packet;

use self::peer::*;
use self::bcast::*;

pub use self::localip::get_localip;

use hardware_io;
use planner;

const PEER_PORT: u16 = 5177;
const BCAST_PORT: u16 = 5176;

pub fn start(send_message_rx: mpsc::Receiver<SendMessageCommand>,
             i_am_stuck_rx: mpsc::Receiver<()>,
             message_recieved_tx: mpsc::Sender<Packet<SendMessageCommand, String>>,
             peer_update_tx: mpsc::Sender<PeerUpdate<String>>)
             -> thread::JoinHandle<()> {
    
    //let unique = rand::thread_rng().gen::<u16>(); //TODO: make deterministic
    
    let unique = 1; //Not so unique, really

    // Creates network thread
    thread::Builder::new().name("network".to_string()).spawn(move || {

        // Creates PeerTransmitter thread
        thread::spawn(move || { 
            let id = format!("{}:{}", get_localip().expect("failed to get local ip"), unique);
            //let id = get_localip().expect("failed to get local ip");
            PeerTransmitter::new(PEER_PORT)
                .expect("Error creating PeerTransmitter")
                .run(i_am_stuck_rx, &id);
        });

        // Creates PeerReciever thread
        //let (peer_tx, peer_rx) = channel::<PeerUpdate<String>>();
        thread::spawn(move || {
            PeerReceiver::new(PEER_PORT)
                .expect("Error creating PeerReceiver")
                .run(peer_update_tx);
        });
        
        // Creates BcastTransmitter thread
        thread::spawn(move || {
            BcastTransmitter::new(BCAST_PORT)
                .expect("Error creating BcastTransmitter")
                .run(send_message_rx);

        });

        // Creates BcastReciever thread
        //let (message_tx, message_rx): (mpsc::Sender<SendMessageCommand>, mpsc::Receiver<SendMessageCommand>) = mpsc::channel();
        thread::spawn(move || {
            BcastReceiver::new(BCAST_PORT)
                .expect("Error creating BcastReciever")
                .run(message_recieved_tx);
        });

        loop {
            /* TEST
            thread::sleep(Duration::from_millis(5000));
            peer_update_hold_tx.send(()).unwrap();
            */
        }
    }).expect("Failed to start thread")
}
