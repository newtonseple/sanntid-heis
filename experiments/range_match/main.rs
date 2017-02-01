fn main() {
    let a = 5;
    match a {
        0 ... (5-1) => println!("succsess"),
        0...6 => println!("succsess+1"),
        _ => println!("Nothing works!"),
    }
}