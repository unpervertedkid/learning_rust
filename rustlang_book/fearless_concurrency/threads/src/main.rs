use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for number in 1..10 {
            println!("Hi number {} from the spawned thread!", number);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for number in 1..10 {
        println!("Hi number {} from the main thread!", number);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
