pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut left_pointer = 0;
    let mut right_pointer = 0;
    let mut len = i32::MAX;
    let mut current_sum = 0;

    while right_pointer < nums.len() {
        current_sum += nums[right_pointer];
        right_pointer += 1;

        while current_sum >= target {
            len = len.min(right_pointer as i32 - left_pointer as i32);

            current_sum -= nums[left_pointer];
            left_pointer += 1;
        }
    }

    if len == i32::MAX {
        return 0 as i32;
    } else {
        return len;
    }
}
