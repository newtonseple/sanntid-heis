use std::process::Command;
use std::time;
use std::thread;
use std::env;
use std::fs;

fn get_state() -> (i32, time::Instant) {
    fs::File::create("test");
    for file in fs::read_dir(".") {
        println!("{:?}", file);
    }
    return (0, time::Instant::now());
}

fn main() {
    get_state();
    /*
    let arg = env::args().nth(1).expect("Did not get a argument");
    println!("{:?}", arg);
    let mut i: i32 = arg.parse().unwrap();
    i += 1;
    if i < 10 {
        let output = Command::new("start")
            .arg("call")
            .arg("target\\debug\\process-pair")
            .arg(format!("{}", i))
            .spawn()
        //    .expect("Failed to spawn process");
        .expect("Failed to execute command");
    } else {
        println!("This is the end: {}", i);
    }
    thread::sleep(time::Duration::from_secs(10));
    */
}
