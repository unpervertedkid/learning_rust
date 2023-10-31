use std::cmp::max;

pub fn jump(nums: Vec<i32>) -> i32 {
    let nums_length = nums.len();
    let mut farthest_reachable_index = 0;
    let mut current_jump_end = 0;
    let mut number_of_jumps = 0;

    for current_index in 0..nums_length - 1 {
        farthest_reachable_index = max(farthest_reachable_index, current_index + nums[current_index] as usize);

        if current_index == current_jump_end {
            number_of_jumps += 1;
            current_jump_end = farthest_reachable_index;
        }
    }

    return number_of_jumps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = jump(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![2, 3, 0, 1, 4];
        let result = jump(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![2, 1];
        let result = jump(nums);
        assert_eq!(result, 1);
    }
}
