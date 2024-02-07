use crate::cell::Cell;
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

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

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Exclusive => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY
        // A Ref is only given out if no exclusive references have been given out.
        // Once it is given, state is set to shared so no exlusive references are given out.
        // So dereferencing into a shared reference is fine.
        unsafe { &*self.refcell.value.get() }
    }
}
pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY
        // See safety for DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    // SAFETY
    // A RefMut is only given out if no other references have been given out.
    // Once it is given, state is set to Exclusive so no future references are given out.
    // So we have an exclusive lease on the inner value so dereferencing is fine .
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}
