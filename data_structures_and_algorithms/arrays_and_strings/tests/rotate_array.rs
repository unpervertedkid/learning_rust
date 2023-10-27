pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}

pub fn rotate_inefficiently(nums: &mut Vec<i32>, k: i32) {
    let vec_length = nums.len();

        for _ in 0..k {
            let mut previous_element = nums[vec_length - 1];
            
            for index in 0..nums.len() {
                let current_element = nums[index];
                nums[index] = previous_element;
                previous_element = current_element;
            }
        }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_case_1() {
        let mut input = vec![1,2,3,4,5,6,7];
        rotate(&mut input, 3);
        assert_eq!(input, vec![5,6,7,1,2,3,4]);
    }

    #[test]
    fn test_case_2() {
        let mut input = vec![-1,-100,3,99];
        rotate(&mut input, 2);
        assert_eq!(input, vec![3,99,-1,-100]);
    }

    #[test]
    fn test_case_3() {
        let mut input = vec![1,2,3,4,5,6,7];
        rotate_inefficiently(&mut input, 3);
        assert_eq!(input, vec![5,6,7,1,2,3,4]);
    }

    #[test]
    fn test_case_4() {
        let mut input = vec![-1,-100,3,99];
        rotate_inefficiently(&mut input, 2);
        assert_eq!(input, vec![3,99,-1,-100]);
    }
}