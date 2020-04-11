use std::collections::{HashSet, HashMap, BTreeMap, BTreeSet};
use std::cmp::{max, min};
use std::mem::swap;

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

#[cfg(test)]
mod tests {
    use super::{longest_substring, max_area, three_sum, next_permutation, multiply_strings, rotate_matrix};

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