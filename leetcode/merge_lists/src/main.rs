#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn print(self) {
        fn helper(maybe_node: Option<Box<ListNode>>) {
            match maybe_node {
                None => (),
                Some(node) => {
                    if let None = node.next {
                        println!("{}]", node.val);
                    } else {
                        print!("{}, ", node.val);
                        helper(node.next);
                    }
                }
            }
        }

        print!("[");
        helper(Some(Box::new(self)));
    }

    fn from_array(arr: &[i32]) -> Option<Box<ListNode>> {
        if arr.len() == 0 {
            return None;
        }

        let node = ListNode {
            val: arr[0],
            next: ListNode::from_array(&arr[1..]),
        };
        Some(Box::new(node))
    }
}

struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(node_a), None) => Some(node_a),
            (None, Some(node_b)) => Some(node_b),
            (Some(node_a), Some(node_b)) => {
                let mut new_node: Box<ListNode>;
                if node_a.val < node_b.val {
                    new_node = Box::new(ListNode::new(node_a.val));
                    new_node.next = Solution::merge_two_lists(node_a.next, Some(node_b));
                } else {
                    new_node = Box::new(ListNode::new(node_b.val));
                    new_node.next = Solution::merge_two_lists(Some(node_a), node_b.next);
                }

                Some(new_node)
            }
        }
    }
}

fn main() {
    let list1 = ListNode::from_array(&[1, 2, 4]);
    let list2 = ListNode::from_array(&[1, 3, 4]);

    let merged = Solution::merge_two_lists(list1, list2);
    merged.unwrap().print();
}
