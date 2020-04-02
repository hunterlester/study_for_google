use std::collections::{HashSet};

pub fn longest_substring(s: String) -> i32 {
    let mut char_set: HashSet<String> = HashSet::new();
    let mut substrings: Vec<i32> = Vec::new();
    let mut iterations = 0;
    for i in 0..s.len() {
        for char in s[i..].chars() {
            iterations += 1;
            if !char_set.insert(char.to_string()) {
                substrings.push(char_set.len() as i32);
                char_set.clear();
                break;
            }
        }
    }
    println!("iterations: {:?}", iterations);
    substrings.sort();
    if let Some (substring_len) = substrings.last() {
        *substring_len
    } else if s.len() == 1 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{longest_substring};

    #[test]
    fn test_longest_substring() {
        let mut s = String::from("b");
        assert_eq!(longest_substring(s), 1);
        s = String::from(" ");
        assert_eq!(longest_substring(s), 1);
        s = String::from("");
        assert_eq!(longest_substring(s), 0);
        s = String::from("aaabbbccc");
        assert_eq!(longest_substring(s), 2);
        s = String::from("abcdag");
        assert_eq!(longest_substring(s), 5);
        s = String::from("bbbbbbbbbb");
        assert_eq!(longest_substring(s), 1);
        s = String::from("abcabcbb");
        assert_eq!(longest_substring(s), 3);
    }

}