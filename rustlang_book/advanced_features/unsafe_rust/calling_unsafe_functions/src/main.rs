use core::slice;

fn main() {
    calling_unsafe_function();
    creating_safe_abstraction_over_unsafe_code();
}

fn calling_unsafe_function() {
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

fn creating_safe_abstraction_over_unsafe_code() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];

    let reference = &mut vector[..];

    let (a, b) = reference.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

#[allow(dead_code)]
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let length = values.len();
    let pointer = values.as_mut_ptr();

    assert!(mid < length);

    unsafe {
        (
            slice::from_raw_parts_mut(pointer, mid),
            slice::from_raw_parts_mut(pointer.add(mid), length - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_vector() {
        let mut vector = vec![1, 2, 3, 4, 5, 6];

        let reference = &mut vector;

        let (a, b) = split_at_mut(reference, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
}
