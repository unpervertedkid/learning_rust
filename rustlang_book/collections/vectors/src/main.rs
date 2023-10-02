fn main() {
    let mut marks:Vec<u32> = vec![10, 20, 30, 40, 50];

    // Accessing elements of a vector directly using the index. Using an index out of 
    // bounds will cause a panic.
    let top = &marks[4];
    println!("The top mark is {}", top);

    // Accessing elements of a vector using the get() method. Using an index out of
    // bounds will return None.
    let bottom = marks.get(0);
    match bottom {
        Some(mark) => println!("The bottom mark is {}", mark),
        None => println!("No bottom mark found"),
    }

    println!("Adding 10 to each mark");
    
    // Iterating over mutable references to each element in the vector. 
    for mark in &mut marks {
        *mark += 10;
    }

    // Iterating over immutable references to each element in the vector.
    for mark in &marks {
        println!("Mark: {}", mark);
    }
}
