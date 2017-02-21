use std::process::Command;

fn main() {
    let output = Command::new("echo").arg("Hello").output().expect("Failed to execute command");
    println!("{:?}", output);
}
