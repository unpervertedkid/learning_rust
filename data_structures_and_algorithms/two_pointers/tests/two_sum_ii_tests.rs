use two_pointers::two_sum_ii::two_sum;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![2,3,4];
        let target = 6;
        assert_eq!(two_sum(numbers, target), vec![1, 3]);
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![-1,0];
        let target = -1;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }
}