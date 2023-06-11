use std::{thread, time};

// Thread function
fn thread_func() {
    loop {
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn main() {

    // Threads
    let thread_1 = thread::spawn(move || thread_func());
    let thread_2 = thread::spawn(move || thread_func());
    let thread_3 = thread::spawn(move || thread_func());

    thread_1.join().unwrap();
    thread_2.join().unwrap();
    thread_3.join().unwrap();
    // thread_func();
}