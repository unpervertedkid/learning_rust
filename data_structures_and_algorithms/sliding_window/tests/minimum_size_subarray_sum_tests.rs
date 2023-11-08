use sliding_window::minimum_size_subarray_sum::min_sub_array_len;

#[test]
fn test_case_1() {
    let target = 7;
    let nums = vec![2, 3, 1, 2, 4, 3];
    let expected = 2;
    let actual = min_sub_array_len(target, nums);
    assert_eq!(actual, expected);
}

#[test]
fn test_case_2() {
    let target = 4;
    let nums = vec![1, 4, 4];
    let expected = 1;
    let actual = min_sub_array_len(target, nums);
    assert_eq!(actual, expected);
}

#[test]
fn test_case_3() {
    let target = 11;
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let expected = 0;
    let actual = min_sub_array_len(target, nums);
    assert_eq!(actual, expected);
}