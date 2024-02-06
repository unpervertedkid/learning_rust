use std::{sync::{Arc, Mutex}, thread};

fn main() {
    simple_single_thread_mutex();
    share_mutex_with_multiple_threads();
}

fn share_mutex_with_multiple_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            let mut counter_in_thread = counter.lock().unwrap();
            *counter_in_thread += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn simple_single_thread_mutex() {
    let mutex = Mutex::new(5);

    {
        let mut number = mutex.lock().unwrap();
        *number = 6;
    }

    println!("mutex = {:?}", mutex);
}
