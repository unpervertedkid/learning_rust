pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut current_target_index = nums.len() - 1;
    let mut can_reach_target = true;

    while current_target_index > 0 && can_reach_target {
        let mut current_search_index:i32 = (current_target_index - 1) as i32;
        can_reach_target = false;

        while current_search_index >= 0 && !can_reach_target {
            let current_search_value = nums[current_search_index as usize];
            let distance_to_target = current_target_index - current_search_index as usize;

            if current_search_value >= distance_to_target as i32 {
                can_reach_target = true;
                current_target_index = current_search_index as usize;
            }

            current_search_index -= 1;
        }
    }

    return can_reach_target;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = can_jump(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 1, 0, 4];
        let result = can_jump(nums);
        assert_eq!(result, false);
    }
}
