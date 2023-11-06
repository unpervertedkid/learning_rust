pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut right_pointer = numbers.len() - 1;
    let mut left_pointer = 0;

    while right_pointer > left_pointer {
        let current_sum = numbers[right_pointer] + numbers[left_pointer];

        if current_sum == target {
            return vec![(left_pointer + 1) as i32, (right_pointer + 1) as i32];
        } else if current_sum < target {
            left_pointer += 1;
        } else {
            right_pointer -= 1;
        }
    }

    return vec![-1, -1];
}
