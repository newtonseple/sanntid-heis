use std::thread;

static NTHREADS: i32 = 10;

static mut i: i32 = 0;

fn main() {
    let thread_1 = thread::spawn(move || {
        for _ in 0..1000000 {
            unsafe {
            i = i - 1;
        }
        }});

    let thread_2 = thread::spawn(move || {
        for _ in 0..1000000 {
            unsafe {
                i = i+1;
            }
        }
    });

    let _ = thread_1.join();
    let _ = thread_2.join();
    unsafe {
        println!("{}",i);
    }
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
