pub fn is_palindrome(s: String) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        let left_char = s.as_bytes()[left] as char;
        let right_char = s.as_bytes()[right] as char;

        if !left_char.is_ascii_alphanumeric() {
            left += 1;
        } else if !right_char.is_ascii_alphanumeric() {
            right -= 1;
        } else if !left_char.eq_ignore_ascii_case(&right_char) {
            return false;
        } else {
            left += 1;
            right -= 1;
        }
    }

    return true;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_case_1() {
        let string = String::from("A man, a plan, a canal: Panama");

        assert_eq!(is_palindrome(string), true);
    }

    #[test]
    fn test_case_2() {
        let string = String::from("race a car");

        assert_eq!(is_palindrome(string), false);
    }

    #[test]
    fn test_case_3() {
        let string = String::from(" ");

        assert_eq!(is_palindrome(string), true);
    }
}