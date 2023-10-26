use std::collections::HashMap;
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut elements = HashMap::new();
    let mut majority: (i32, i32) = (-1, -1);

    for number in nums {
        let mut current_count = 1;
        if elements.contains_key(&number) {
            current_count = elements.get(&number).unwrap() + 1;
        }
        if current_count > majority.1 {
            majority.0 = number;
            majority.1 = current_count;
        }

        elements.insert(number, current_count);
    }

    return majority.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = vec![3,2,3];
        let output = majority_element(input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_case_2() {
        let input = vec![2,2,1,1,1,2,2];
        let output = majority_element(input);
        assert_eq!(output, 2);
    }
}