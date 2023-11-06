use two_pointers::is_subsequence::is_subsequence;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");

        assert_eq!(is_subsequence(s, t), true);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");

        assert_eq!(is_subsequence(s, t), false);
    }
}