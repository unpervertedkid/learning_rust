use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: We know no one else is concurently mutating self.value (because !sync)
        // SAFETY: We know we're not invalidating any references cause we're not giving any out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: We know that no one else is modifying this value, since only this thread can
        // mutate it (!Sync) and its executing this function instead
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn test_set_get() {
        let c = Cell::new(5);
        assert_eq!(c.get(), 5);
        c.set(10);
        assert_eq!(c.get(), 10);
    }

    #[test]
    fn test_safety_of_get() {
        let c = Cell::new(5);
        let _ = c.get();
        // If we got here, then no panic occurred and it's safe to use get
    }
}
