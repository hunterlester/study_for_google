use std::collections::{HashSet, HashMap, BTreeMap, BTreeSet};
use std::cmp::{max, min};
use std::mem::swap;
use std::ops::Bound::{Included, Excluded};

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

// no good as exceeds time limit when n is large
// fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut a: usize = 0;
//     let mut b: usize = 1;
//     let mut answer_set: BTreeSet<Vec<i32>> = BTreeSet::new();
//     let mut output = Vec::new();
//     println!("nums len: {:?}", nums.len());
//     while nums.len() >= 3 && a <= nums.len() - 3 {
//         println!("a: {:?}, b: {:?}", a, b);
//         for v in &nums[b + 1..] {
//             if nums[a] + nums[b] + v == 0 {
//                 let mut triplet = vec![nums[a], nums[b], *v];
//                 triplet.sort();
//                 answer_set.insert(triplet);
//             }
//             if b == nums.len() - 2 {
//                 a += 1;
//                 b = a;
//             }
//         }
//         b += 1;
//     }
//     for vec in answer_set.iter() {
//         output.push(vec.clone());
//     }
//     output
// }

// dead end
// fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
//     println!("{:?}", &nums);
//     let mut output = Vec::new();
//     if nums.len() < 3 {
//         return output;
//     }
//     let mut a: usize = 0;
//     let mut b: usize = nums.len() - 1;
//     let mut answer_set: BTreeSet<Vec<i32>> = BTreeSet::new();
//     let mut value_map: HashMap<i32, usize> = HashMap::new();
//     for (i, value) in nums.iter().enumerate() {
//         value_map.insert(*value, i);
//     }
//     nums.sort();
//     while a < b {
//         let a_value = nums[a];
//         let b_value = nums[b];
//         let c_value = -(a_value + b_value);
//         println!("a: {:?}, b: {:?}", a, b);
//         println!("a_value: {:?}, b_value: {:?}, c_value: {:?}", a_value, b_value, c_value);
//         let is_zero_sum = a_value + b_value + c_value == 0;
//         if value_map.contains_key(&c_value) && is_zero_sum {
//             let mut triplet = vec![a_value, b_value, c_value];
//             triplet.sort();
//             answer_set.insert(triplet);
//         }
//         a += 1;
//         b -= 1;
//     }
//     for vec in answer_set.iter() {
//         output.push(vec.clone());
//     }
//     output
// }

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer_set: BTreeSet<Vec<i32>> = BTreeSet::new();
    let mut output = Vec::new();
    let mut value_map: HashMap<i32, i32> = HashMap::new();
    let mut a: usize = 0;
    let mut b: usize = 1;
    for v in nums.iter() {
        if let Some(count) = value_map.get_mut(v) {
            *count += 1;
        } else {
            value_map.insert(*v, 1);
        }
    }

    while nums.len() >= 3 && a <= nums.len() - 2 {
        let a_value = nums[a];
        let b_value = nums[b];
        let complement = -(a_value + b_value);
        let mut duplicate_count = 0;
        if complement == a_value {
            duplicate_count += 1;
        }
        if complement == b_value {
            duplicate_count += 1;
        }
        if let Some(count) = value_map.get(&complement) {
            if *count > duplicate_count {
                let mut triplet = vec![a_value, b_value, complement];
                triplet.sort();
                answer_set.insert(triplet);
            }
        }
        if b == nums.len() -1 {
            a += 1;
            b = a;
        }
        b += 1;
    }
    for vec in answer_set.iter() {
        output.push(vec.clone());
    }
    output
}

fn next_permutation(nums: &mut Vec<i32>) {
    if nums.len() < 2 {
        return;
    }
    let mut a: usize = nums.len() - 2;
    let mut b: usize = nums.len() - 1;
    while a > 0 && nums[a + 1] <= nums[a] {
        a -= 1;
    }
    while b > 0 && nums[b] <= nums[a] {
        b -= 1;
    }
    if a == 0 && b == 0 {
        nums.reverse();
    } else {
        nums.swap(a, b);
        nums[a + 1..].sort();
    }
}

// whoa, holy mackerel!
// fn multiply_strings(num1: String, num2: String) -> String {
//     if num1 == String::from("0") || num2 == String::from("0") {
//         return String::from("0");
//     }
//     let mut num1_as_bytes: Vec<u8> = num1.into_bytes().iter_mut().map(|byte| *byte - 48).collect();
//     let mut num2_as_bytes: Vec<u8> = num2.into_bytes().iter_mut().map(|byte| *byte - 48).collect();
//     let mut byte_output: Vec<u8> = Vec::new();
// 
//     let mut num2_index: usize = num2_as_bytes.len() - 1;
//     let mut sum_queue: Vec<Vec<u8>> = Vec::new();
//     let mut carry: u8 = 0;
//     let mut longest_sum_array_len = 0;
//     loop {
//         let mut num1_index: usize = num1_as_bytes.len() - 1;
//         let mut sum_sub_array = Vec::new();
//         loop {
//             let mut product = num1_as_bytes[num1_index] * num2_as_bytes[num2_index];
//             product += carry;
//             carry = 0;
//             if product > 9 {
//                 let ones_values = product % 10;
//                 let tens_value = (product - (ones_values)) / 10;
//                 carry = tens_value;
//                 product = ones_values;
//                 if num1_index == 0 {
//                     sum_sub_array.push(product);
//                     sum_sub_array.push(carry);
//                     carry = 0;
//                     break;
//                 }
//             }
//             sum_sub_array.push(product);
//             if num1_index == 0 {
//                 break;
//             }
//             num1_index -= 1;
//         }
//         for _i in  0..num2_as_bytes.len() - 1 - num2_index {
//             sum_sub_array.insert(0, 0);
//         }
//         longest_sum_array_len = max(sum_sub_array.len(), longest_sum_array_len);
//         sum_queue.push(sum_sub_array);
//         if num1_index == 0 && num2_index == 0 {
//             break;
//         }
//         num2_index -= 1;
//     }
// 
//     for i in 0..longest_sum_array_len {
//         let mut sum = 0;
//         sum += carry;
//         carry = 0;
//         for vec in &sum_queue {
//             if let Some(value) = vec.get(i) {
//                 sum += value;
//                 if sum > 9 {
//                     let ones_values = sum % 10;
//                     let tens_value = (sum - (ones_values)) / 10;
//                     carry += tens_value;
//                     sum = ones_values;
//                     if i == longest_sum_array_len - 1 {
//                         byte_output.insert(0, sum);
//                         byte_output.insert(0, carry);
//                         sum = 0;
//                         carry = 0;
//                         break;
//                     }
//                 }
//             } else {
//                 continue;
//             }
//         }
//         byte_output.insert(0, sum);
//     }
//     if byte_output[0] == 0 {
//         byte_output.remove(0);
//     }
// 
//     byte_output = byte_output.iter_mut().map(|int| *int + 48).collect();
//     String::from_utf8(byte_output).unwrap()
// }

fn multiply_strings(num1: String, num2: String) -> String {
    let num1_as_u8: Vec<u8> = num1.into_bytes().iter_mut().map(|byte| *byte - 48).collect();
    let num2_as_u8: Vec<u8> = num2.into_bytes().iter_mut().map(|byte| *byte - 48).collect();
    let num1_len = num1_as_u8.len();
    let num2_len = num2_as_u8.len();
    let mut sum: Vec<u8> = vec![0; num1_len + num2_len]; 
    let mut pointer1: usize = 0;
    let mut pointer2: usize = 0; 
    for num1_index in (0..num1_len).rev() {
        for num2_index in (0..num2_len).rev() {
            pointer1 = num1_index + num2_index;
            pointer2 = pointer1 + 1;
            let mut product = num1_as_u8[num1_index] * num2_as_u8[num2_index];
            product += sum[pointer2];
            let ones_value = product % 10;
            let mut tens_value = sum[pointer1] + ((product - ones_value) / 10);
            if let Some(value) = sum.get_mut(pointer2) {
                *value = ones_value;
            }
            if let Some(value) = sum.get_mut(pointer1) {
                *value = tens_value;
            }
        }
    }
    loop {
        if let Some(int) = sum.get(0) {
            if *int == 0 && sum.len() > 1 {
                sum.remove(0);
            } else {
                break;
            }
        }
    }
    sum = sum.iter_mut().map(|int| *int + 48).collect();
    String::from_utf8(sum).unwrap()
}

fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) {
    let mut pairs_to_swap: Vec<(usize, usize)> = Vec::new();
    for (a, matrix_member_array) in matrix.iter().enumerate() {
        for (b, _array_member) in matrix_member_array.iter().enumerate() {
            if a < b {
                pairs_to_swap.push((a, b));
            }
        }
    }
    for (a, b) in pairs_to_swap {
        let (mut part_1, mut part_2) = matrix.split_at_mut(a + 1);
        let part_1_len = part_1.len();
        swap(&mut part_1[a][b], &mut part_2[b - part_1_len][a]);
    }
    for matrix_member_array in matrix.iter_mut() {
        matrix_member_array.reverse();
    }
}

fn can_jump(nums: Vec<i32>) -> bool {
    let mut bool_index: Vec<bool> = vec![false; nums.len()];
    for (i, value) in nums.iter().enumerate().rev() {
        if i + 1 == nums.len() {
            if let Some(v) = bool_index.get_mut(i) {
                *v = true;
            }
        } else {
            for jump_index in 1..=*value {
                let jump_index_bool = bool_index[i + jump_index as usize];
                if let Some(bool) = bool_index.get_mut(i) {
                    *bool = jump_index_bool;
                    if jump_index_bool == true {
                        break;
                    }
                }
            }
        }
    }
    bool_index[0]
}

fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    for i in (0..digits.len()).rev() {
        if carry == 0 {
            break;
        }
        if let Some(digit) = digits.get_mut(i) {
            let sum = *digit + carry;
            let ones_place = sum % 10;
            carry = (sum - ones_place) / 10;
            *digit = ones_place;
        }
        if i == 0 && carry > 0 {
            digits.insert(0, carry);
        }
    }
    digits
}

fn min_window(s: String, t: String) -> String {
    let mut target_char_count: HashMap<String, i32> = HashMap::new();
    let mut char_index: HashMap<String, Vec<usize>> = HashMap::new();
    let mut char_index_set: BTreeSet<usize> = BTreeSet::new();
    let mut smallest_target_window_indices: Option<(usize, usize)> = None;
    let mut window_boundary: usize = 0;
    for char in t.chars() {
        if let Some(count) = target_char_count.get_mut(&char.to_string()) {
            *count += 1;
        } else {
            target_char_count.insert(char.to_string(), 1);
        }
    }
    for (i, char) in s.char_indices() {
        if let Some(count) = target_char_count.get(&char.to_string()) {
            if let Some(vec) = char_index.get_mut(&char.to_string()) {
                vec.push(i);
                char_index_set.insert(i);
                let char_index_len = vec.len();
                if char_index_len > *count as usize {
                    let item = vec.remove(0);
                    char_index_set.remove(&item);
                    if let Some(index) = char_index_set.iter().next() {
                        window_boundary = *index;
                    }
                }
            } else {
                char_index.insert(char.to_string(), vec![i]);
                char_index_set.insert(i);
                if let Some(index) = char_index_set.iter().next() {
                    window_boundary = *index;
                }
            }
            if char_index_set.len() == t.len() {
                if let Some((index_a, index_b)) = smallest_target_window_indices {
                    if (index_b - index_a) > (i - window_boundary) {
                        smallest_target_window_indices = Some((window_boundary, i))
                    }
                } else {
                    smallest_target_window_indices = Some((window_boundary, i));
                }
                char_index_set.remove(&window_boundary);
                if let Some(index) = char_index_set.iter().next() {
                    window_boundary = *index;
                }
            }
        }
    }
    if let Some((start, end)) = smallest_target_window_indices {
        String::from(&s[start..=end])
    } else {
        String::from("")
    }
}

fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    const DISTINCT_CHARS: i8 = 2;
    let mut longest_substring: i32 = 0;
    let mut char_index: HashMap<String, usize> = HashMap::with_capacity(DISTINCT_CHARS as usize + 1);
    let mut left: usize = 0; 
    let mut right: usize = 0; 
    while right < s.len() {
        char_index.insert(String::from(&s[right..right+1]), right);
        let (least_i_char, least_index) = if let Some((least_i_char, least_index)) = char_index.iter().min_by(|x, y| x.1.cmp(y.1)) { (least_i_char.clone(), *least_index) } else { break; };
        if char_index.len() > DISTINCT_CHARS as usize {
            longest_substring = max(longest_substring, right as i32 - left as i32);
            char_index.remove(&least_i_char);
            left = least_index + 1;
        } else {
            longest_substring = max(longest_substring, right as i32 - left as i32 + 1);
        }
        right += 1;
    }
    longest_substring
}

// fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
//     let mut output: Vec<String> = Vec::new();
//     if nums.len() == 0 {
//         if (upper - lower) > 1 {
//             output.push(format!("{:?}->{:?}", lower, upper));
//         } else {
//             output.push(format!("{:?}", lower));
//         }
//         return output;
//     }
//     for (i, num) in nums.iter().enumerate() {
//         if i == 0 {
//             let range_count = num - lower;
//             if range_count > 1 {
//                 output.push(format!("{:?}->{:?}", lower, num - 1));
//             } else if range_count == 1 {
//                 output.push(format!("{:?}", lower));
//             }
//             if nums.len() == 1 {
//                 break;
//             }
//         }
// 
//         if *num == upper {
//             return output;
//         }
// 
//         let next_num = nums[i + 1];
//         let range_count = next_num - num;
//         if range_count > 2 {
//             output.push(format!("{:?}->{:?}", num + 1, next_num - 1));
//         } else if range_count == 2 {
//             output.push(format!("{:?}", num + 1));
//         }
//         if i == nums.len() - 2 {
//             break;
//         }
//     }
// 
//     let next_num = nums[nums.len() - 1];
//     let range_count = upper - next_num;
//     if range_count > 1 {
//         output.push(format!("{:?}->{:?}", next_num + 1, upper));
//     } else if range_count == 1 {
//         output.push(format!("{:?}", upper));
//     }
//     output
// }

fn find_missing_ranges(nums: Vec<i32>, mut lower: i32, upper: i32) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for num in nums {
        if num > lower {
            if num - 1 > lower {
                output.push(format!("{:?}->{:?}", lower, num - 1));
            } else {
                output.push(format!("{:?}", lower));
            } 
        }
        if num == upper {
            return output;
        }
        lower = num + 1;
    }
    if upper > lower {
        output.push(format!("{:?}->{:?}", lower, upper));
    } else if upper == lower {
        output.push(format!("{:?}", lower));
    }
    output
}

fn next_closest_time(time: String) -> String {
    let mut digit_values: BTreeSet<u32> = BTreeSet::new();
    let mut digit_order: Vec<u32> = Vec::new();
    let mut min_value: u32 = 9;
    for char in time.chars() {
        if let Some(int) = char.to_digit(10) {
            digit_order.push(int);
            digit_values.insert(int);
            min_value = min(min_value, int);
        }
    }
    for (i, digit) in digit_order.iter_mut().enumerate().rev() {
        let mut upper_bound = Included(&9);
        if i == 2 {
            upper_bound = Included(&5);
        }
        if i == 1 {
            if let Ok(int) = time[0..1].parse::<u32>() {
                if int == 2 {
                    upper_bound = Included(&3);
                }
            }
        }
        if i == 0 {
            upper_bound = Included(&2);
            if let Ok(int) = time[1..2].parse::<u32>() {
                if int > 3 {
                    upper_bound = Included(&1);
                }
            }
        }
        if let Some(next_value) = digit_values.range((Excluded(&*digit), upper_bound)).next() {
            *digit = *next_value;
            break;
        } else {
            *digit = min_value;
        }
    }
    format!("{:?}{:?}:{:?}{:?}", digit_order[0], digit_order[1], digit_order[2], digit_order[3])
}

#[cfg(test)]
mod tests {
    use super::{
        longest_substring, max_area, three_sum, next_permutation, multiply_strings,
        rotate_matrix, can_jump, plus_one, min_window, length_of_longest_substring_two_distinct,
        find_missing_ranges, next_closest_time
    };

    #[test]
    fn test_next_closest_time() {
        let mut time = String::from("19:34");
        assert_eq!(next_closest_time(time), String::from("19:39"));
        time = String::from("15:35");
        assert_eq!(next_closest_time(time), String::from("15:51"));
        time = String::from("12:55");
        assert_eq!(next_closest_time(time), String::from("15:11"));
        time = String::from("12:55");
        assert_eq!(next_closest_time(time), String::from("15:11"));
        time = String::from("02:55");
        assert_eq!(next_closest_time(time), String::from("05:00"));
        time = String::from("23:55");
        assert_eq!(next_closest_time(time), String::from("22:22"));
    }

    #[test]
    fn test_find_missing_ranges() {
        let mut nums = vec![0, 1, 3, 50, 75];
        let mut upper = 99;
        let mut lower = 0;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["2", "4->49", "51->74", "76->99"]);
        nums = vec![3, 50, 75];
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["0->2", "4->49", "51->74", "76->99"]);
        nums = vec![];
        upper = 1;
        lower = 1;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["1"]);
        nums = vec![];
        upper = 99;
        lower = 1;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["1->99"]);
        nums = vec![];
        upper = 0;
        lower = 0;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["0"]);
        nums = vec![-9, -5, -1, 3, 6, 15];
        upper = 20;
        lower = -12;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["-12->-10", "-8->-6", "-4->-2", "0->2", "4->5", "7->14", "16->20"]);
        nums = vec![-1];
        upper = -1;
        lower = -1;
        assert_eq!(find_missing_ranges(nums, lower, upper), Vec::<String>::new());
        nums = vec![-1];
        lower = -2;
        upper = -1;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["-2"]);
        nums = vec![-2648, 2647];
        lower = -2648;
        upper = 2647;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["-2647->2646"]);
        nums = vec![0, 1, 3, 50, 75, 80, 90];
        upper = 75;
        lower = 0;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["2", "4->49", "51->74"]);
        nums = vec![-2147483648,2147483647];
        lower = -2147483648;
        upper = 2147483647;
        assert_eq!(find_missing_ranges(nums, lower, upper), vec!["-2147483647->2147483646"]);
    }

    #[test]
    fn test_length_of_longest_substring_two_distinct() {
        let mut string = String::from("eceba");
        assert_eq!(length_of_longest_substring_two_distinct(string), 3);
        string = String::from("ccaabbb");
        assert_eq!(length_of_longest_substring_two_distinct(string), 5);
        string = String::from("aaaaaa");
        assert_eq!(length_of_longest_substring_two_distinct(string), 6);
        string = String::from("");
        assert_eq!(length_of_longest_substring_two_distinct(string), 0);
        string = String::from("ab");
        assert_eq!(length_of_longest_substring_two_distinct(string), 2);
        string = String::from("a");
        assert_eq!(length_of_longest_substring_two_distinct(string), 1);
        string = String::from("abaccc");
        assert_eq!(length_of_longest_substring_two_distinct(string), 4);
    }

    #[test]
    fn test_min_window() {
        let mut string = String::from("ADOBECODEBANC");
        let mut target = String::from("ABC");
        assert_eq!(min_window(string, target), String::from("BANC"));
        string = String::from("ADOBECODEBANC");
        target = String::from("");
        assert_eq!(min_window(string, target), String::from(""));
        string = String::from("ADOBECODEBANC");
        target = String::from("JKL");
        assert_eq!(min_window(string, target), String::from(""));
        string = String::from("a");
        target = String::from("a");
        assert_eq!(min_window(string, target), String::from("a"));
        string = String::from("aa");
        target = String::from("aa");
        assert_eq!(min_window(string, target), String::from("aa"));
        string = String::from("bbaa");
        target = String::from("aba");
        assert_eq!(min_window(string, target), String::from("baa"));
        string = String::from("a");
        target = String::from("aa");
        assert_eq!(min_window(string, target), String::from(""));
        string = String::from("a");
        target = String::from("b");
        assert_eq!(min_window(string, target), String::from(""));
        string = String::from("abab");
        target = String::from("aa");
        assert_eq!(min_window(string, target), String::from("aba"));
        string = String::from("aabbcce");
        target = String::from("abc");
        assert_eq!(min_window(string, target), String::from("abbc"));
        string = String::from("ABRADOBECODEBANC");
        target = String::from("ABBC");
        assert_eq!(min_window(string, target), String::from("BRADOBEC"));
        string = String::from("ab");
        target = String::from("b");
        assert_eq!(min_window(string, target), String::from("b"));
    }

    #[test]
    fn test_plus_one() {
        let mut digits = vec![9, 9, 9];
        assert_eq!(plus_one(digits), vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_can_jump() {
        let mut nums = vec![2, 3, 1, 1, 4];
        assert_eq!(can_jump(nums), true);
        nums = vec![3, 2, 1, 0, 4];
        assert_eq!(can_jump(nums), false);
        nums = vec![0];
        assert_eq!(can_jump(nums), true);
        nums = vec![0, 0];
        assert_eq!(can_jump(nums), false);
        nums = vec![4, 0];
        assert_eq!(can_jump(nums), true);
    }

    fn matrices_eq(a: &mut Vec<Vec<i32>>, b: &mut Vec<Vec<i32>>) -> bool {
        let flat_matrix_a: Vec<&i32> = a.iter().flatten().collect();
        let flat_matrix_b: Vec<&i32> = b.iter().flatten().collect();
        flat_matrix_a.iter().enumerate().all(|(i, int)| *int == flat_matrix_b[i])
    }

    #[test]
    fn test_rotate_matrix() {
        let mut matrix = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];
        rotate_matrix(&mut matrix);
        let mut expected_matrix = vec![
            vec![7,4,1],
            vec![8,5,2],
            vec![9,6,3],
        ];
        assert!(matrices_eq(&mut matrix, &mut expected_matrix));

        let mut matrix_2 = vec![
            vec![ 5, 1, 9,11],
            vec![ 2, 4, 8,10],
            vec![13, 3, 6, 7],
            vec![15,14,12,16],
        ];
        rotate_matrix(&mut matrix_2);
        let mut expected_matrix_2 = vec![
            vec![15,13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7,10,11],
        ];
        assert!(matrices_eq(&mut matrix_2, &mut expected_matrix_2));
    }

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

    #[test]
    fn test_three_sum() {
        let mut nums = vec![-1, 0, 1, 2, -1, -4];
        let mut output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(three_sum(nums), output);
        nums = vec![];
        output = vec![];
        assert_eq!(three_sum(nums), output);
        nums = vec![0];
        output = vec![];
        assert_eq!(three_sum(nums), output);
        nums = vec![0, 0];
        output = vec![];
        assert_eq!(three_sum(nums), output);
        nums = vec![1,2,-2,-1];
        output = vec![];
        assert_eq!(three_sum(nums), output);
        nums = vec![-1, 4, 0, 2, -1, -4];
        output = vec![vec![-4, 0, 4], vec![-1, -1, 2]];
        assert_eq!(three_sum(nums), output);
        nums = vec![-1, 0, 1, 0];
        output = vec![vec![-1, 0, 1]];
        assert_eq!(three_sum(nums), output);
        nums = vec![0, 0, 0];
        output = vec![vec![0, 0, 0]];
        assert_eq!(three_sum(nums), output);
    }

    #[test]
    fn test_next_permutation() {
        let mut number: Vec<i32> = vec![1, 3, 2];
        next_permutation(&mut number);
        assert_eq!(number, vec![2, 1, 3]);
        number = vec![1, 2, 5, 4, 3];
        next_permutation(&mut number);
        assert_eq!(number, vec![1, 3, 2, 4, 5]);
        number = vec![1];
        next_permutation(&mut number);
        assert_eq!(number, vec![1]);
        number = vec![];
        next_permutation(&mut number);
        assert_eq!(number, vec![]);
        number = vec![0,0,0];
        next_permutation(&mut number);
        assert_eq!(number, vec![0,0,0]);
        number = vec![1, 4, 4];
        next_permutation(&mut number);
        assert_eq!(number, vec![4, 1, 4]);
        number = vec![2, 3, 1];
        next_permutation(&mut number);
        assert_eq!(number, vec![3, 1, 2]);
        number = vec![3, 2, 1];
        next_permutation(&mut number);
        assert_eq!(number, vec![1, 2, 3]);
        number = vec![4,2,0,2,3,2,0];
        next_permutation(&mut number);
        assert_eq!(number, vec![4,2,0,3,0,2,2]);
    }

    #[test]
    fn test_multiply_strings() {
        let mut num1 = String::from("162");
        let mut num2 = String::from("41");
        assert_eq!(multiply_strings(num1, num2), String::from("6642"));
        num1 = String::from("41");
        num2 = String::from("162");
        assert_eq!(multiply_strings(num1, num2), String::from("6642"));
        num1 = String::from("0");
        num2 = String::from("0");
        assert_eq!(multiply_strings(num1, num2), String::from("0"));
        num1 = String::from("0");
        num2 = String::from("144");
        assert_eq!(multiply_strings(num1, num2), String::from("0"));
        num1 = String::from("721");
        num2 = String::from("140");
        assert_eq!(multiply_strings(num1, num2), String::from("100940"));
        num1 = String::from("3");
        num2 = String::from("38");
        assert_eq!(multiply_strings(num1, num2), String::from("114"));
    }
}