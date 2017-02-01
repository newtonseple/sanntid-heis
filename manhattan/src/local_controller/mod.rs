
pub enum LocalEventMessage {
    TimerTick,
    ArrivedAtFloor { floor: i32 },
}

pub fn start(local_event_rx: mpsc::Receiver<LocalEventMessage>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        select! {
    		local_event_result = local_event_rx.recv() => {
    			match local_event_result.unwrap() {
    				unimplemented!()
    			}
    		}
    	}
    })
}