use std::collections::{HashSet, BTreeMap};
use std::ops::Bound::{Unbounded, Excluded};

/// Assumptions:
/// Each passed email contains exactly one '@' character.
/// All local and domain names are non-empty.
/// Local names do not start with a '+' character.
pub fn unique_email_addresses(emails: Vec<String>) -> u32 {
    let mut unique_addresses = HashSet::new();
    for email in &emails {
        let local_domain_split: Vec<&str> = email.split("@").collect();
        let mut local = local_domain_split[0];
        let local_plus_char_index = local.find("+");
        if let Some(index) = local_plus_char_index {
            local = &local[0..index];
        }
        let local_portion = &local.split(".").collect::<Vec<&str>>().join("");
        unique_addresses.insert([local_portion, local_domain_split[1]].join("@"));
    }
    unique_addresses.len() as u32
}

// Time complexity is too great
pub fn odd_even_jump_iterative(number_array: Vec<i16>) -> u16 {
    let mut count = 1;
    for i in 0..number_array.len() - 1 {
        let mut index = i;
        let mut jump = 1;
        loop {
            let is_odd_jump = jump % 2 != 0;
            if is_odd_jump {
                let nums_ge_index: Vec<&i16> = number_array[index + 1..].iter().filter(|n| n >= &&number_array[index]).collect();
                if let Some(min_value) = nums_ge_index.iter().min() {
                    if let Some(jump_index) = number_array.iter().enumerate().position(|(i, &element)| element == **min_value && i > index) {
                        index = jump_index;
                        jump += 1;
                    }
                } else {
                    break;
                }
            } else {
                let nums_le_index: Vec<&i16> = number_array[index + 1..].iter().filter(|n| n <= &&number_array[index]).collect();
                if let Some(max_value) = nums_le_index.iter().max() {
                    if let Some(jump_index) = number_array.iter().enumerate().position(|(i, &element)| element == **max_value && i > index) {
                        index = jump_index;
                        jump += 1;
                    }
                } else {
                    break;
                }
            }
            if index == number_array.len() - 1 {
                count += 1;
                break;
            }
        }
    }
    count
}
// [2, 3, 1, 1, 4]
// pub fn odd_even_jump(a: Vec<i32>) -> () {
//     let last_arr_index = a.len() - 1;
//     let mut good_odd_jumps: Vec<bool> = Vec::with_capacity(a.len());
//     let mut good_even_jumps: Vec<bool> = Vec::with_capacity(a.len());
//     let mut index_map: BTreeMap<i32, usize> = BTreeMap::new();
//     good_odd_jumps[a.len() - 1] = true;
//     good_even_jumps[a.len() - 1] = true;
//     index_map.insert(a[a.len() - 1], a.len() - 1);
// 
//     for i in [0..last_arr_index].iter().rev() {
//         if let Some((index_value, _index)) = index_map.get(a[i]) {
//             good_even_jumps.insert(i, good_odd_jumps[index_value]);
//             good_odd_jumps.insert(i, good_even_jumps[index_value]);
//         } else {
//             let next_higher_value = index_map.range((Excluded(a[i]), Unbounded)).next();
//             let last_lower_value = index_map.range(a[i]..).next_back();
//             println!("next_higher_value: {:?}", &next_higher_value);
//             println!("last_lower_value: {:?}", &last_lower_value);
// 
//             if let Some((higher_value, _index)) = next_higher_value {
//                 good_odd_jumps.insert(i, good_even_jumps[higher_value]);
//             }
// 
//             if let Some((lower_value, _index)) = last_lower_value {
//                 good_even_jumps.insert(i, good_odd_jumps[lower_value]);
//             }
// 
//             index_map.insert(a[i], i);
//         }
//     }
//     println!("odd index: {:?}", &good_odd_jumps);
//     println!("even index: {:?}", &good_even_jumps);
// }

// pub fn monotonic_odd_even_jump(a: Vec<i32>) {
//     let mut sorted_a = a.clone();
//     sorted_a.sort();
//     let mut a_index_map: HashMap<i32, Vec<usize>> = HashMap::new();
//     for (index, sorted_value) in sorted_a.iter().enumerate() {
//         if let Some(index_vec) = a_index_map.get_mut(&sorted_value) {
//             index_vec.insert(0, index);
//         } else {
//             a_index_map.insert(*sorted_value, vec![index]);
//         }
//     }
// 
//     let mut a_as_sorted_indexes = Vec::new();
//     for value in a {
//         if let Some(vector) = a_index_map.get_mut(&value) {
//             if let Some(popped_value) = vector.pop() {
//                 a_as_sorted_indexes.push(popped_value);
//             }
//         }
//     }
// 
//     let mut a_as_sorted_indexes_map: BTreeMap<i32, usize> = BTreeMap::new();
//     for (i, v) in a_as_sorted_indexes.iter().enumerate() {
//         a_as_sorted_indexes_map.insert(*v as i32, i);
//     }
//     println!("{:?}", &a_as_sorted_indexes_map);
//     ()
// }

fn format_license_key(s: String, k: i32) -> String {
    let mut new_key: Vec<String> = Vec::with_capacity(s.len());
    let mut group_index = 0;
    for char in s.chars().rev() {
        if group_index == k {
            if char.to_string() == "-" {
                new_key.push(char.to_string());
                group_index = 0;
            } else {
                new_key.push(String::from("-"));
                new_key.push(char.to_uppercase().to_string());
                group_index = 1;
            }
        } else {
            if char.to_string() == "-" {
                continue;
            } else {
                new_key.push(char.to_uppercase().to_string());
                group_index += 1;
            }
        }
    }
    if let Some(char) = new_key.last() {
        if char == "-" {
            new_key.pop();
        }
    }
    new_key.join("").chars().rev().collect()
}

// fn fruit_into_baskets(tree: Vec<i32>) -> i32 {
//     let mut fruit_sums: Vec<i32> = Vec::with_capacity(tree.len());
//     let mut index: Option<usize> = Some(0);
//     let mut basket_one_count = 0;
//     let mut basket_two_count = 0;
//     while let Some(i) = index {
//         let mut fruit_one: Option<i32> = None;
//         let mut fruit_two: Option<i32> = None;
//         let mut fruit_two_index: Option<usize> = None;
//         for (fruit_index, fruit) in tree[i..].iter().enumerate() {
//             if let Some(int) = fruit_one {
//                 if *fruit == int {
//                     basket_one_count += 1;
//                 }
//             } else {
//                 fruit_one = Some(*fruit);
//                 basket_one_count += 1;
//             }
// 
//             if let Some(int) = fruit_two {
//                 if *fruit == int {
//                     basket_two_count += 1;
//                 } else if Some(*fruit) != fruit_one {
//                     fruit_sums.push(basket_one_count + basket_two_count);
//                     basket_one_count = 0;
//                     basket_two_count = 0;
//                     if let Some(starting_fruit_two_index) = fruit_two_index {
//                         index = Some(starting_fruit_two_index);
//                     }
//                     break;
//                 }
//             } else {
//                 if Some(*fruit) != fruit_one {
//                     fruit_two_index = Some(i + fruit_index);
//                     fruit_two = Some(*fruit);
//                     basket_two_count += 1;
//                 }
//             }
// 
//             if i + fruit_index == tree.len() - 1 {
//                 fruit_sums.push(basket_one_count + basket_two_count);
//                 basket_one_count = 0;
//                 basket_two_count = 0;
//                 index = None;
//                 break;
//             }
//         }
//     }
//     fruit_sums.sort();
//     fruit_sums[fruit_sums.len() - 1]
// }

fn fruit_into_baskets(tree: Vec<i32>) -> i32 {
    let mut sub_trees: Vec<i32> = Vec::new();
    let mut index: Option<usize> = Some(0);
    while let Some(i) = index {
        let mut fruit_one: Option<i32> = None;
        let mut fruit_two: Option<i32> = None;
        let mut fruit_two_first_index: Option<usize> = None;
        for (fruit_index, fruit) in tree[i..].iter().enumerate() {
            if let None = fruit_one {
                fruit_one = Some(*fruit);
            }

            if let Some(int) = fruit_two {
                if *fruit != int && Some(*fruit) != fruit_one {
                    sub_trees.push(tree[i..i + fruit_index].len() as i32);
                    fruit_one = None;
                    fruit_two = None;
                    index = fruit_two_first_index;
                    break;
                }
            } else {
                if Some(*fruit) != fruit_one {
                    fruit_two = Some(*fruit);
                    fruit_two_first_index = Some(i + fruit_index);
                }
            }

            if i + fruit_index == tree.len() - 1 {
                sub_trees.push(tree[i..=i + fruit_index].len() as i32);
                fruit_one = None;
                fruit_two = None;
                index = None;
                break;
            }
        }
    }
    sub_trees.sort();
    sub_trees[sub_trees.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::{unique_email_addresses, odd_even_jump_iterative, format_license_key, fruit_into_baskets};
    #[test]
    fn test_unique_email_addresses() {
        let emails = vec![
            String::from("vartan.vartabed.kalustian+email@gmail.com"),
            String::from("vartanvartabedkalustian+email@gmail.com"),
            String::from("vartan.vartabedkalustian+email@gmail.com"),
            String::from("vartanvartabedkalustian@gmail.com"),
            String::from("vartanvartabed.kalustian@gmail.com"),
        ];
        assert_eq!(unique_email_addresses(emails), 1);
    }

    #[test]
    fn test_odd_even_jump() {
        let num_array = vec![10, 13, 12, 14, 15];
        assert_eq!(odd_even_jump_iterative(num_array), 2);
    }

    #[test]
    fn test_odd_even_jump_1() {
        let num_array = vec![2, 3, 1, 1, 4];
        assert_eq!(odd_even_jump_iterative(num_array), 3);
    }

    #[test]
    fn test_odd_even_jump_2() {
        let num_array = vec![5, 1, 3, 4, 2];
        assert_eq!(odd_even_jump_iterative(num_array), 3);
    }

    // #[test]
    // fn test_improved_odd_even_jump() {
    //     let num_array = vec![2, 3, 1, 1, 4];
    //     odd_even_jump(num_array);
    // }

    #[test]
    fn test_format_license_key() {
        let key = "1-5Wyz-9e-t5";
        let grouping_size = 4;
        assert_eq!(format_license_key(String::from(key), grouping_size), "1-5WYZ-9ET5");
    }

    #[test]
    fn test_format_license_key_2() {
        let key = "--a-a-a-a--";
        let grouping_size = 2;
        assert_eq!(format_license_key(String::from(key), grouping_size), "AA-AA");
    }

    #[test]
    fn test_fruit_into_baskets() {
        let tree = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
        assert_eq!(fruit_into_baskets(tree), 5)
    }

    #[test]
    fn test_fruit_into_baskets_2() {
        let tree = vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4];
        assert_eq!(fruit_into_baskets(tree), 11)
    }

    #[test]
    fn test_fruit_into_baskets_3() {
        let tree = vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
        assert_eq!(fruit_into_baskets(tree), 11)
    }

    #[test]
    fn test_fruit_into_baskets_4() {
        let tree = vec![3];
        assert_eq!(fruit_into_baskets(tree), 1)
    }

    #[test]
    fn test_fruit_into_baskets_5() {
        let tree = vec![0, 1, 6, 6, 4, 4, 6];
        assert_eq!(fruit_into_baskets(tree), 5)
    }

    #[test]
    fn test_fruit_into_baskets_6() {
        let tree = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(fruit_into_baskets(tree), 2)
    }
}
