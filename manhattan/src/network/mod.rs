use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;
use std::sync::mpsc::channel;

use rand;
use rand::Rng;
use chrono::offset::local::Local;

pub mod localip;
mod bcast;
mod peer;
use self::peer::*;

use self::localip::get_localip;

use hardware_io;
use planner;

const PEER_PORT: u16 = 9877;
const BCAST_PORT: u16 = 9876;

pub enum SendMessageCommand {
    IAmStuck,
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
        id: usize,
    }, // usize for use in array indexing, other types might be more appropriate
}

pub fn start(send_message_rx: mpsc::Receiver<SendMessageCommand>,
             message_recieved_tx: mpsc::Sender<planner::RecievedMessage>,
             peer_update_tx: mpsc::Sender<planner::PeerUpdate>)
             -> thread::JoinHandle<()> {
    
    let unique = rand::thread_rng().gen::<u16>(); //TODO: make deterministic

    thread::spawn(move || {
        let (peer_update_hold_tx, peer_update_hold_rx) = mpsc::channel();
        thread::spawn(move || {
            let id = format!("{}:{}", get_localip().unwrap(), unique);
            PeerTransmitter::new(PEER_PORT)
                .expect("Error creating PeerTransmitter")
                .run(peer_update_hold_rx,&id);
        });

        let (peer_tx, peer_rx) = channel::<PeerUpdate<String>>();
        thread::spawn(move || {
            PeerReceiver::new(PEER_PORT)
                .expect("Error creating PeerReceiver")
                .run(peer_tx);
        });
        //TODO: START MESSAGE PASSERS
        loop {
            select! {
                peer_rx_result = peer_rx.recv() => {
                    //unimplemented!();
                },
                //TODO: ADD MESSAGE HANDLERS (recieve message)
                send_message_result = send_message_rx.recv() => {
                   let send_message = send_message_result.unwrap();
                    println!("got send message!!");
                }
            }
            /* TEST
            thread::sleep(Duration::from_millis(5000));
            peer_update_hold_tx.send(()).unwrap();
            */
        }
    })
}
