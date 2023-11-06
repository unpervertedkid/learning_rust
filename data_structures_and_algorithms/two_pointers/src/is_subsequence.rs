pub fn is_subsequence(s: String, t: String) -> bool {
    let mut s_pointer = 0;
    let mut t_pointer = 0;

    while s_pointer < s.len() && t_pointer < t.len() {
        if s.as_bytes()[s_pointer as usize] == t.as_bytes()[t_pointer as usize] {
            s_pointer += 1;
        }

        t_pointer += 1;
    }

    return s_pointer == s.len();
}
