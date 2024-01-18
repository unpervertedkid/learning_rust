

struct Counter {
    count: usize,
    up_to: usize,
}

// The counter should count from 0 to the specified up to
impl Counter {
    fn new(up_to: usize) -> Counter {
        Counter {
            count: 0,
            up_to: up_to,
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    // Next implementation
    // We should yield new items, incrementing by 1 until we reach up t0
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        // Check to see if we've reached up_to
        if self.count > self.up_to {
            None
        } else {
            Some(self.count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yields_elements_upto() {
        let mut counter = Counter::new(5);

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
    
    #[test]
    fn does_not_yield_when_upto_is_0() {
        let mut counter = Counter::new(0);

        assert_eq!(counter.next(), None);
    }
}