use std::{thread, time::Duration, vec};

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

    // Using move closures with threads
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
