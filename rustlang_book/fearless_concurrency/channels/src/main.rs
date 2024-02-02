use std::{sync::mpsc, thread, time::Duration};

fn main() {
    separator();
    send_values_across_threads();

    separator();
    send_multiple_values();

    separator();
    send_using_multiple_producers();
}

fn send_using_multiple_producers() {
    let (transmitter, receiver) = mpsc::channel();

    let transmitter_1 = transmitter;
    let transmitter_2 = transmitter_1.clone();

    thread::spawn(move || {
        let values = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for value in values {
            transmitter_1.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for value in values {
            transmitter_2.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    

    for received in receiver {
        println!("Got: {received}");
    }
}

fn send_values_across_threads() {
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

fn send_multiple_values() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for value in values {
            transmitter.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("Got: {received}");
    }
}

fn separator() {
    println!("------------------------------------------------------------");
}
