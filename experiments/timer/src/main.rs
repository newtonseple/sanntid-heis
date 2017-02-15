use std::time::Duration;
use std::thread::sleep;

fn main() {
    loop {
        println!("Tick");
        sleep(Duration::from_millis(10));
    }
}