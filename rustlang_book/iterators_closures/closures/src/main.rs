use std::thread;

fn main() {
    divider();
    println!("Capturing references and moving ownership");
    capture_immutably();
    divider();

    capture_mutably();
    divider();

    take_ownership();
    divider();

    println!("Moving captured values out of closures and the Fn traits");
    fn_mut_trait();
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    _height: u32,
}

fn fn_mut_trait() {
    let mut list = [
        Rectangle {
            width: 30,
            _height: 50,
        },
        Rectangle {
            width: 10,
            _height: 20,
        },
        Rectangle {
            width: 40,
            _height: 60,
        },
    ];
    
    println!("Before sorting: {:#?}", list);

    list.sort_by_key(|rectangle| rectangle.width);
    
    println!("After sorting: \n{:#?}", list);
}

fn divider() {
    println!("------------------------------------------------");
}
