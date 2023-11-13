use two_pointers::substring_with_concatenation_of_all_words::find_substring;

#[test]
fn test_case_1() {
    let s = String::from("barfoothefoobarman");
    let words = vec![String::from("foo"), String::from("bar")];
    let result = find_substring(s, words);
    assert_eq!(result, vec![0, 9]);
}

#[test]
fn test_case_2() {
    let s = String::from("wordgoodgoodgoodbestword");
    let words = vec![
        String::from("word"),
        String::from("good"),
        String::from("best"),
        String::from("word"),
    ];
    let result = find_substring(s, words);
    assert_eq!(result, vec![]);
}

#[test]
fn test_case_3() {
    let s = String::from("barfoofoobarthefoobarman");
    let words = vec![
        String::from("bar"),
        String::from("foo"),
        String::from("the"),
    ];
    let result = find_substring(s, words);
    assert_eq!(result, vec![6, 9, 12]);
}