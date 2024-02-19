fn main() {
    matching_literals();
    matching_named_variables();
    multiple_patterns();
    matching_ranges();

    destructing_structs();
    destructing_enums();
}

fn matching_literals() {
    let number = 1;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Anything else"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {y}", x);
}

fn multiple_patterns() {
    let number = 1;

    match number {
        1 | 2 => println!("One or two"),
        3 => println!("Three"),
        _ => println!("Anything else"),
    }
}

fn matching_ranges() {
    let number = 4;

    match number {
        1..=5 => println!("One through five"),
        _ => println!("Anthing else"),
    }
}

// Destructing to break apart values

struct Point {
    x: i32,
    y: i32,
}

fn destructing_structs() {
    let point = Point { x: 0, y: 7 };

    let Point { x, y } = point;

    assert_eq!(0, x);
    assert_eq!(7, y);

    match point {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}

// Destructing enums

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructing_enums() {
    let message = Message::ChangeColor(0, 160, 255);

    match message {
        Message::Quit => println!("Quitting...."),
        Message::Move { x, y } => {
            println!("Moving to direction x: {x} and y: {y}");
        }
        Message::Write(text) => {
            println!("Writing message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Changing color to red {r}, green {g}, blue {b}");
        }
    }
}
