use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;


fn main() {
    let i = Arc::new(Mutex::new(0));

    let i1 = i.clone();
    let thread_1 = thread::spawn(move || {
        for _ in 0..1000000 {
            let mut data = i1.lock().unwrap();
            *data += 1;
        }
    });

    let i2 = i.clone();
    let thread_2 = thread::spawn(move || {
        for _ in 0..1000000 {
            let mut data = i2.lock().unwrap();
            *data -= 1;
        }
    });

    let _ = thread_1.join();
    let _ = thread_2.join();
    println!("hei", );
    println!("{}",*i.lock().unwrap());
}


/*
// treg, og det hender at den krasjer. Usikker på hvem som eier i
// fungerer neppe
fn main() {
    let i = 0;
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let tx1_c = tx1.clone();
    let tx2_c = tx2.clone();

    let thread1 = thread::spawn(move || {
        for _ in 0..1000000 {
            let mut data = rx1.recv().unwrap();
            println!("{}", i);
            data += 1;
            tx2_c.send(data).unwrap();
        }
    });

    let thread2 = thread::spawn(move || {
        for _ in 0..1000000 {
            let mut data = rx2.recv().unwrap();
            println!("{}", i);
            data -= 1;
            tx1_c.send(data).unwrap();
        }
    });
    tx1.send(i).unwrap();
    let _ = thread1.join();
    let _ = thread2.join();
    println!("ferdig", );
    println!("{}", i);
}
*/

/*
// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
*/
