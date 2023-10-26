pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut last_valid_index = 0;
    let mut next_valid_index = 0;

    while next_valid_index < nums.len() {
        let current_value  = nums[next_valid_index as usize];
        
        
        //If there exists more than one element, copy the first two elements and find the next valid index(index that is not a repeat of the current number)
        if next_valid_index + 1 < nums.len() && current_value == nums[(next_valid_index + 1) as usize] {
            //copy the first two elements to last valid index
            nums[last_valid_index as usize] = current_value;
            last_valid_index += 1;
            next_valid_index += 1;

            nums[last_valid_index as usize] = nums[next_valid_index as usize];
            last_valid_index += 1;
            next_valid_index += 1;

            //Find next valid index
            while next_valid_index < nums.len() && current_value == nums[next_valid_index as usize] {
                next_valid_index += 1;
            }
            // Otherwise just copy the current number to the last valid index and increment the both the last valid index and the current valid index
        } else {
            nums[last_valid_index as usize] = current_value;
            last_valid_index += 1;
            next_valid_index += 1;
        }
    }

    return last_valid_index as i32;
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_case_1() {
    let mut input = vec![1,1,1,2,2,3];
    let output = remove_duplicates(&mut input);
    assert_eq!(output, 5);
    assert_eq!(input[..5], vec![1,1,2,2,3]);
}

#[test]
fn test_case_2() {
    let mut input = vec![0,0,1,1,1,1,2,3,3];
    let output = remove_duplicates(&mut input);
    assert_eq!(output, 7);
    assert_eq!(input[..7], vec![0,0,1,1,2,3,3]);
}
}