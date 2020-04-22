
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }

    // fn insert(&mut self, val: i32) {
    //     let mut current_node;
    //     if let Some(boxed_node) = &mut self.next {
    //         current_node = &mut *boxed_node;
    //     } else {
    //         self.next = Some(Box::new(Self::new(val)));
    //         return;
    //     };
    //     loop {
    //         if let Some(boxed_node) = &mut current_node.next {
    //             current_node = &mut *boxed_node;
    //             continue;
    //         } else {
    //             current_node.next = Some(Box::new(Self::new(val)));
    //             break;
    //         }
    //     }
    // }
}

fn build_linked_list(mut array: Vec<i32>) -> Option<Box<ListNode>> {
    if let Some(val) = array.pop() {
        Some(Box::new(ListNode {
            val,
            next: build_linked_list(array),
        }))
    } else { None }
}

pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut sum_vec: Vec<i32> = Vec::new();
    loop {
        let mut sum = 0;
        sum += carry;
        carry = 0;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }
        let ones_place = sum % 10;
        let tens_place = (sum - ones_place) / 10;
        carry = tens_place;
        sum_vec.insert(0, ones_place);
        if let None = l1 {
            if let None = l2 {
                if carry == 0 {
                    break;
                }
            }
        }
    }
    build_linked_list(sum_vec)
}

#[cfg(test)]
mod tests {
    use super::{add_two_numbers, ListNode};

    #[test]
    fn test_add_two_numbers() {
        let mut l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                }))
            }))
        };
        let mut l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                }))
            }))
        };
        let mut expected = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 8,
                    next: None,
                }))
            }))
        }));
        let mut output = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        assert_eq!(output, expected);
        l1 = ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: None,
                }))
            }))
        };
        l2 = ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: None
            }))
        };
        expected = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: None,
                    })),
                }))
            }))
        }));
        output = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        assert_eq!(output, expected);
        l1 = ListNode {
            val: 0,
            next: None, 
        };
        l2 = ListNode {
            val: 0,
            next: None, 
        };
        expected = Some(Box::new(ListNode {
            val: 0,
            next: None
        }));
        output = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        assert_eq!(output, expected);
    }
}