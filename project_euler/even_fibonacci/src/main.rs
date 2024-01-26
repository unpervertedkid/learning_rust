struct Fibonacci {
    current: i32,
    next: i32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current = self.next;
        self.next += current;

        Some(current)
    }
}

fn main() {
    let sum: i32 = Fibonacci::new()
        .take_while(|&number| number <= 4000000)
        .filter(|&number| number % 2 == 0)
        .sum();

    println!("The sum of all even fibonacci numbers whose value is less 4 million is: {sum}")
}
