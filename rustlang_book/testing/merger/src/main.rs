fn merge(mut a: Vec<i32>, b: Vec<i32>, a_size: i32, b_size: i32 ) -> Vec<i32> {
    let mut a_end = a_size - 1;
    let mut b_end = b_size - 1;
    let mut merged_end = a_size + b_size - 1;

    while b_end >= 0 {
        if a_end >= 0 && a[a_end as usize] > b[b_end as usize] {
            a[merged_end as usize] = a[a_end as usize];
            a_end -= 1;
        } else {
            a[merged_end as usize] = b[b_end as usize];
            b_end -= 1;
        }
        merged_end -= 1;
    }

    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let a = vec![1, 3, 5, 7, 9, 0, 0, 0, 0, 0];
        let b = vec![2, 4, 6, 8, 10];
        let result = merge(a, b, 5, 5);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test] 
    fn test_equal_size() {
        let a = vec![1, 3, 5, 7, 9, 0, 0, 0, 0, 0];
        let b = vec![2, 4, 6, 8, 10];
        let result = merge(a, b, 5, 5);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}