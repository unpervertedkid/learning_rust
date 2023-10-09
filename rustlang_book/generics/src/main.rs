struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1,Y1> Point<X1,Y1> {
    fn mixup<X2, Y2> (self, other: Point<X2,Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let number_list = vec![34,57,87,64,98,100];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point { x: 5, y: 10.4};
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
