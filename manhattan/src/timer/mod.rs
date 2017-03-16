/*
This is a simple timer, providing tick events to local_controller.
*/

use std::thread;
use std::thread::sleep;
use std::sync::mpsc::Sender;
use std::time::Duration;

use local_controller;

pub fn run(timer_tx: Sender<local_controller::LocalEventMessage>) -> thread::JoinHandle<()> {
    thread::Builder::new()
        .name("timer".to_string())
        .spawn(move || loop {
            timer_tx.send(local_controller::LocalEventMessage::TimerTick)
                .expect("Could not send timer tick");
            sleep(Duration::from_millis(100));
        })
        .expect("Failed to start thread")
}
