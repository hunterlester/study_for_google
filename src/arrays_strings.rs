use std::collections::{HashSet, HashMap};
use std::cmp::max;

// pub fn longest_substring(s: String) -> i32 {
//     let mut char_set: HashSet<String> = HashSet::new();
//     let mut substrings: Vec<i32> = Vec::new();
//     let mut iterations = 0;
//     for i in 0..s.len() {
//         for char in s[i..].chars() {
//             iterations += 1;
//             if !char_set.insert(char.to_string()) {
//                 substrings.push(char_set.len() as i32);
//                 char_set.clear();
//                 break;
//             }
//         }
//     }
//     println!("iterations: {:?}", iterations);
//     substrings.sort();
//     if let Some (substring_len) = substrings.last() {
//         *substring_len
//     } else if s.len() == 1 {
//         1
//     } else {
//         0
//     }
// }

// pub fn longest_substring(s: String) -> i32 {
//         let n = s.len();
//         let mut set = HashSet::new();
//         let mut ans = 0;
//         let mut i = 0;
//         let mut j = 0;
//         let mut iterations = 0;
//         while (i < n && j < n) {
//             println!("j: {:?}, i: {:?}", j, i);
//             iterations += 1;
//             // try to extend the range [i, j]
//             let char = if let Some(slice_index) = s.get(j..j + 1) {
//                 slice_index
//             } else {
//                 &s[j..j + 1]
//             };
//             if (set.insert(char)) {
//                 j += 1;
//                 ans = max(ans, j as i32 - i as i32);
//             }
//             else {
//                 let char = if let Some(slice_index) = s.get(i..i + 1) {
//                     slice_index
//                 } else {
//                     &s[i..i + 1]
//                 };
//                 set.remove(char);
//                 i += 1;
//             }
//         }
//         println!("iterations: {:?}", iterations);
//         ans as i32
// }

pub fn longest_substring(s: String) -> i32 {
    let mut char_map: HashMap<String, usize> = HashMap::new();
    let mut starting_boundary: usize = 0;
    let mut longest_substring_len: usize = 0;
    for (ending_boundary, char) in s.char_indices() {
        if let Some(char_index) = char_map.get(&char.to_string()) {
            starting_boundary = max(*char_index, starting_boundary);
        }
        longest_substring_len = max(longest_substring_len, ending_boundary - starting_boundary + 1);
        char_map.insert(char.to_string(), ending_boundary + 1);
    }
    longest_substring_len as i32
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
        s = String::from("abcdefghijkl");
        assert_eq!(longest_substring(s), 12);
        s = String::from("abcdefghijkk");
        assert_eq!(longest_substring(s), 11);
        s = String::from("abcc");
        assert_eq!(longest_substring(s), 3);
    }

}