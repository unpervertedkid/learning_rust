use two_pointers::container_with_most_water::max_area;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn test_case_2() {
        let height = vec![1,1];
        assert_eq!(max_area(height), 1);
    }

    #[test]
    fn test_case_3() {
        let height = vec![4,3,2,1,4];
        assert_eq!(max_area(height), 16);
    }
}
