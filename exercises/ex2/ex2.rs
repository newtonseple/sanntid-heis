use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

/*
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
*/


enum Cmd {
    Inc,
    Dec,
    Fin,
}

fn main() {
    let mut i = 0;
    let (server_tx, server_rx) = mpsc::channel();
    let (fin_tx, fin_rx) = mpsc::channel();
    let tx1 = server_tx.clone();
    let tx2 = server_tx.clone();

    let _ = thread::spawn(move || {
        let mut num_fin = 0;
        loop {
            if num_fin == 2 {
                fin_tx.send(i).unwrap();
                break;
            }
            match server_rx.recv().unwrap() {
                Cmd::Inc => i += 1,
                Cmd::Dec => i -= 1,
                Cmd::Fin => num_fin += 1,
            }
        }
    });

    let _ = thread::spawn(move || {
        for _ in 0..1000000 {
            tx1.send(Cmd::Inc).unwrap();
        }
        tx1.send(Cmd::Fin).unwrap();
    });

    let _ = thread::spawn(move || {
        for _ in 0..1000000 {
            tx2.send(Cmd::Dec).unwrap();
        }
        tx2.send(Cmd::Fin).unwrap();
    });

    let i = fin_rx.recv().unwrap();
    println!("ferdig", );
    println!("{}", i);
}


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
