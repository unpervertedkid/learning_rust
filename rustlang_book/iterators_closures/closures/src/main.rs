use std::thread;

fn main() {
    capture_immutably();
    divider();
    
    capture_mutably();
    divider();
    
    take_ownership();
    divider();
}

fn capture_immutably() {
    println!("Capturing values immutably");
    let list = vec![1, 2, 3];

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn capture_mutably() {
    println!("Capturing values mutably");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn take_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .expect("Failed to join thread");
}
fn divider() {
    println!("------------------------------------------------");
}