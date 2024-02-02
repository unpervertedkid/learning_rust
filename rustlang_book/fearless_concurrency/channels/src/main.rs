use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_nanos(1));

        let value = String::from("Hello");
        transmitter.send(value).unwrap();
    });

    loop {
        let maybe_received = receiver.try_recv();

        if maybe_received.is_ok() {
            let received = maybe_received.unwrap();
            println!("Got: {received}");
            break;
        } else {
            println!("Doing other work.....");
        }
    }    
}
