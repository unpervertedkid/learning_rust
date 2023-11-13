use std::collections::HashMap;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    if words.is_empty() || s.is_empty() {
        return vec![];
    }

    let word_len = words[0].len();
    let all_words_len = word_len * words.len();
    let mut word_count = HashMap::new();

    for word in &words {
        let count = word_count.entry(word.clone()).or_insert(0);
        *count += 1;
    }

    let mut result = Vec::new();

    for i in 0..=s.len() - all_words_len {
        let mut seen = HashMap::new();
        let mut j = i;

        while j < i + all_words_len {
            let part = &s[j..j + word_len];
            if !word_count.contains_key(part) {
                break;
            }

            let count = seen.entry(part.to_string()).or_insert(0);
            *count += 1;

            if seen[part] > word_count[part] {
                break;
            }

            j += word_len;
        }

        if j == i + all_words_len {
            result.push(i as i32);
        }
    }

    result
}