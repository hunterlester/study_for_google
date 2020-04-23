
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

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut list_node_pointers: Vec<Box<ListNode>> = Vec::new();
    let mut current_node = head;
    while let Some(boxed_node) = current_node {
        list_node_pointers.push(boxed_node.clone());
        current_node = boxed_node.next;
    }

    if n as usize == list_node_pointers.len() {
        if n > 1 {
            return Some(list_node_pointers[1].clone());
        } else {
            return None;
        }
    }

    let mut pointer_1 = list_node_pointers.len() - n as usize - 1;
    let mut pointer_2 = pointer_1 + 2;

    if let Some(_node_1) = list_node_pointers.get(pointer_1) {
        if let Some(_node_2) = list_node_pointers.get(pointer_2) {
            list_node_pointers[pointer_1].next = Some(list_node_pointers[pointer_2].clone());
        } else {
            list_node_pointers[pointer_1].next = None;
        }
    }

    while pointer_1 >= 1 {
        list_node_pointers[pointer_1 - 1].next = Some(list_node_pointers[pointer_1].clone());
        pointer_1 -= 1;
    }

    Some(list_node_pointers[0].clone())
}

#[cfg(test)]
mod tests {
    use super::{add_two_numbers, ListNode, remove_nth_from_end};

    #[test]
    fn test_remove_nth_from_end() {
        let mut l1 = Some(Box::new(ListNode {
            val: 1,
            next: None, 
        }));
        let mut expected = Some(Box::new(ListNode {
            val: 1,
            next: None, 
        }));
        assert_eq!(remove_nth_from_end(l1, 0), expected);
        l1 = Some(Box::new(ListNode {
            val: 1,
            next: None, 
        }));
        expected = None;
        assert_eq!(remove_nth_from_end(l1, 1), expected);
        l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                })),
            }))
        }));
        expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None,
            }))
        }));
        assert_eq!(remove_nth_from_end(l1, 1), expected);
        l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                })),
            }))
        }));
        expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None,
            }))
        }));
        assert_eq!(remove_nth_from_end(l1, 3), expected);
        let mut l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            }))
        }));
        let mut expected = Some(Box::new(ListNode {
            val: 4,
            next: None, 
        }));
        assert_eq!(remove_nth_from_end(l1, 2), expected);
        let mut l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            }))
        }));
        let mut expected = Some(Box::new(ListNode {
            val: 2,
            next: None, 
        }));
        assert_eq!(remove_nth_from_end(l1, 1), expected);
        l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None,
                        })),
                    })),
                }))
            }))
        }));
        expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: None,
                    })),
                }))
            }))
        }));
        assert_eq!(remove_nth_from_end(l1, 2), expected);
    }

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