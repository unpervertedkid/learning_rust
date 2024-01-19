struct Fibonacci {
    current: u32,
    next: u32,
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
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_first_five_fibonacci_sequence_correctly() {
        let first_five: Vec<u32> = Fibonacci::new().take(5).collect();

        assert_eq!(first_five, vec![0, 1, 1, 2, 3]);
    }
}
