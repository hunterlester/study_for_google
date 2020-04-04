use std::collections::{HashSet, HashMap, BTreeMap};
use std::cmp::{max, min};

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

// O(4n)
// pub fn max_area(height: Vec<i32>) -> i32 {
//     let mut max_container = 0;
//     let mut iterations = 0;
//     for (index_one, coordinate_one) in height.iter().enumerate() {
//         if index_one == height.len() - 1 {
//             break;
//         }
//         for (index_two, coordinate_two) in height[index_one + 1..].iter().enumerate() {
//             iterations += 1;
//             let second_index = index_one + index_two + 1;
//             println!("index_one: {:?}, index_two: {:?}", index_one, second_index);
//             let width = second_index - index_one;
//             let height = min(coordinate_one, coordinate_two);
//             max_container = max(max_container, width as i32 * *height);
//         }
//     }
//     println!("iterations: {:?}", iterations);
//     println!("max_container: {:?}", max_container);
//     max_container
// }

// O(n)
pub fn max_area(height: Vec<i32>) -> i32 {
    if height.len() == 0 {
        return 0;
    }
    let mut starting_index: usize = 0;
    let mut closing_index: usize = height.len() - 1;
    let mut max_area: i32 = 0;
    while starting_index < closing_index {
        let width = closing_index - starting_index;
        let length = min(height[closing_index], height[starting_index]);
        max_area = max(max_area, length * width as i32);
        if height[closing_index] > height[starting_index] {
            starting_index += 1;
        } else {
            closing_index -= 1;
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::{longest_substring, max_area};

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
    
    #[test]
    fn test_max_area() {
        let mut height = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(max_area(height), 49);
        height = vec![1,3,2,5,4,8,6,7,8];
        assert_eq!(max_area(height), 25);
        height = vec![1,3,2,50,60,8,6,7,8];
        assert_eq!(max_area(height), 50);
        height = vec![1];
        assert_eq!(max_area(height), 0);
        height = vec![];
        assert_eq!(max_area(height), 0);
    }
}