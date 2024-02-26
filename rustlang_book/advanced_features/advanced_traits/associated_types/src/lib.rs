mod adder;

pub struct Counter {
    current: usize,
}

impl Counter {
    fn new() -> Self {
        Counter { current: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current += 1;

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_to_five() {
        let mut counter = Counter::new();

        let counter_values: Vec<usize> = counter.take(6).collect();
        assert_eq!(vec![0, 1, 2, 3, 4, 5], counter_values);
    }
}
