use std::thread::sleep;
use std::sync::mpsc::Sender;
use std::time::Duration;

use local_controller;

pub fn start(timer_tx: Sender) {
    thread::Builder::new().name("timer".to_string()).spawn(move || {
        loop {
            timer_tx.send(local_controller::LocalEventMessage::TimerTick)
            sleep(Duration::from_millis(100));
        }
    }).expect("Failed to start thread")
}