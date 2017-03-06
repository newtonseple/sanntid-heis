use std::sync::mpsc;
use std::thread;

use hardware_io;
use network;
use planner;

pub enum LocalEventMessage {
    TimerTick,
    ArrivedAtFloor { floor: i32 },
}

pub enum LocalCommandMessage {
    GoUp,
    GoDown,
    DoNothing,
    StopForOrder { order_type: hardware_io::OrderType },
}

pub fn start(local_event_rx: mpsc::Receiver<LocalEventMessage>,
             hw_command_tx: mpsc::Sender<hardware_io::HwCommandMessage>,
             send_message_tx: mpsc::Sender<network::SendMessageCommand>,
             local_command_request_tx: mpsc::SyncSender<planner::LocalCommandRequestMessage>,
             i_am_stuck_tx: mpsc::Sender<()>,
             local_command_rx: mpsc::Receiver<LocalCommandMessage>)
             -> thread::JoinHandle<()> {
    thread::Builder::new().name("local_controller".to_string()).spawn(move || loop {
        select! {
    		local_event_result = local_event_rx.recv() => {
    				let local_event = local_event_result.unwrap();
    				match local_event {
    				    LocalEventMessage::TimerTick => {
    				    	println!("local_controller got timer tick");
    				    },
    				    LocalEventMessage::ArrivedAtFloor{floor} => {
    				    	println!("local_controller got arrived, {}",floor);
                            println!("Sending i_am_stuck as a test");
                            i_am_stuck_tx.send(()).expect("Error sending 3862386898372443");
    				    },
    				}
    		}
    	}
    }).expect("Failed to start thread")
}
