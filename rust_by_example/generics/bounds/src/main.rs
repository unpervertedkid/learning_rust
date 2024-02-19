use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(item: &T) {
    println!("{:?}", item);
}

fn area<T: HasArea>(item: &T) -> f64 {
    item.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Areaa of rectangle: {}", area(&rectangle));
}
