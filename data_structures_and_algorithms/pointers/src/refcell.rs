use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

// Implied by UnsafeCell
//impl<T> !Sync for RefCell<T> {}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<&T> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: No exclusive references have been given out since state would be Exclusive
                Some(unsafe { &*self.value.get() })
            }
            RefState::Shared(n) => {
                // SAFETY: No exclusive references have been given out since state would be Exclusive
                self.state.set(RefState::Shared(n + 1));
                Some(unsafe { &*self.value.get() })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY: No other references have been given out since state would be either Exclusive
            // or Shared
            Some(unsafe { &mut *self.value.get() })
        } else {
            None
        }
    }
}
