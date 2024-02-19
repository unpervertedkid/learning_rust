use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(item: &T) {
    println!("Debug: `{:?}`", item);
    println!("Display: `{}`", item);
}

fn compare_types<T: Debug, U: Debug>(item_1: &T, item_2: &U) {
    println!("Item 1: `{:?}`", item_1);
    println!("Item 2: `{:?}`", item_2);
}

fn main() {
    let string = "some words";
    let array = [1, 2, 3];
    let vector = vec![1, 2, 3];

    compare_prints(&string);

    compare_types(&array, &vector);
}
