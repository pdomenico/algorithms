use rand::Rng;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

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

    fn print(&self) {
        fn helper(node: &Option<Box<ListNode>>) {
            match node {
                None => (),
                Some(n) => {
                    print!(", {}", n.val);
                    helper(&n.next);
                }
            }
        }

        print!("[{}", self.val);
        helper(&self.next);
        println!("]");
    }
}

fn new_list(arr: &[i32]) -> Option<Box<ListNode>> {
    if arr.is_empty() {
        return None;
    }

    Some(Box::new(ListNode {
        val: arr[0],
        next: new_list(&arr[1..]),
    }))
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut all_empty = true;
        for list in &lists {
            if let Some(_) = list {
                all_empty = false;
            }
        }
        if all_empty {
            return None;
        }

        // First pass merge the lists into vectors and handle the last one if the n of lists is odd
        let mut vectors = Vec::new();
        let is_even = lists.len() % 2 == 0;
        let merge_number = if is_even {
            lists.len() / 2
        } else {
            (lists.len() - 1) / 2
        };

        for i in 0..merge_number {
            let mut merged = Vec::new();
            let (mut ptr_a, mut ptr_b) = (&lists[i * 2], &lists[i * 2 + 1]);
            loop {
                match (ptr_a, ptr_b) {
                    (Some(node_a), Some(node_b)) => {
                        if node_a.val < node_b.val {
                            merged.push(node_a.val);
                            ptr_a = &node_a.next;
                        } else {
                            merged.push(node_b.val);
                            ptr_b = &node_b.next;
                        }
                    }
                    (Some(node_a), None) => {
                        merged.push(node_a.val);
                        ptr_a = &node_a.next;
                    }
                    (None, Some(node_b)) => {
                        merged.push(node_b.val);
                        ptr_b = &node_b.next;
                    }
                    _ => break,
                }
            }
            vectors.push(merged);
        }
        if !is_even {
            let mut last_vec = Vec::new();
            let mut ptr = &lists[lists.len() - 1];
            loop {
                match ptr {
                    Some(node) => {
                        last_vec.push(node.val);
                        ptr = &node.next;
                    }
                    None => break,
                }
            }
            vectors.push(last_vec);
        }

        println!("Vectors after first merge: {:?}", vectors);

        while vectors.len() > 2 {
            let (vec_a, vec_b) = (vectors.swap_remove(0), vectors.swap_remove(0));
            let (mut i, mut j) = (0, 0);
            let mut new_vec = Vec::with_capacity(vec_a.len() + vec_b.len());
            while (i < vec_a.len()) && (j < vec_b.len()) {
                match &vec_a[i].cmp(&vec_b[j]) {
                    std::cmp::Ordering::Less => {
                        new_vec.push(vec_a[i]);
                        i += 1;
                    }
                    _ => {
                        new_vec.push(vec_b[j]);
                        j += 1;
                    }
                }
            }
            while i < vec_a.len() {
                new_vec.push(vec_a[i]);
                i += 1;
            }
            while j < vec_b.len() {
                new_vec.push(vec_b[j]);
                j += 1;
            }
            vectors.push(new_vec);
        }

        println!("{:?}", vectors);

        let mut head = Some(Box::new(ListNode {
            val: vectors[0][0],
            next: None,
        }));
        let mut ptr = &mut head;
        let (mut i, mut j) = (0, 0);
        while (i < vectors[0].len()) && (j < vectors[1].len()) {
            match &vectors[0][i].cmp(&vectors[1][j]) {
                std::cmp::Ordering::Less => {
                    if let Some(node) = ptr {
                        node.next = Some(Box::new(ListNode {
                            val: vectors[0][i],
                            next: None,
                        }));
                        ptr = &mut node.next;
                        i += 1;
                    }
                }
                _ => {
                    if let Some(node) = ptr {
                        node.next = Some(Box::new(ListNode {
                            val: vectors[1][j],
                            next: None,
                        }));
                        ptr = &mut node.next;
                        j += 1;
                    }
                }
            }
        }
        while i < vectors[0].len() {
            if let Some(node) = ptr {
                node.next = Some(Box::new(ListNode {
                    val: vectors[0][i],
                    next: None,
                }));
                ptr = &mut node.next;
                i += 1;
            }
        }
        while j < vectors[1].len() {
            if let Some(node) = ptr {
                node.next = Some(Box::new(ListNode {
                    val: vectors[1][j],
                    next: None,
                }));
                ptr = &mut node.next;
                j += 1;
            }
        }

        head
    }
}

fn main() {
    // Create n lists with random m integers inside them
    const N: usize = 4;
    const M: usize = 10;
    let mut lists: Vec<Option<Box<ListNode>>> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..N {
        let mut numbers = [0; M];
        for i in 0..M {
            numbers[i] = rng.gen_range(1..20);
        }
        numbers.sort();
        lists.push(new_list(&numbers));
    }

    for list in &lists {
        if let Some(node) = list {
            node.print();
        }
    }

    println!("-----------");
    Solution::merge_k_lists(lists);
}
