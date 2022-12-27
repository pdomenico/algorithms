use std::collections::HashMap;
use std::rc::Rc;

struct Solution();

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results_map = HashMap::new();
        let mut result = Vec::new();
        // Put the numbers in hashmap
        let mut numbers_map = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            numbers_map.insert(*n, i);
        }

        for (i, n1) in nums.iter().enumerate() {
            for (j, n2) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }

                let n3 = 0 - (*n1 + *n2);
                if let Some(k) = numbers_map.get(&n3) {
                    if *k == i || *k == j {
                        continue;
                    }
                    let mut triplet = vec![*n1, *n2, n3];
                    triplet.sort();

                    if results_map.contains_key(triplet) {
                        continue;
                    }
                    result.push(triplet.clone());

                    results_map.insert(Box::new(&triplet), true);
                }
            }
        }

        result
    }
}

fn main() {
    let res = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
    println!("{:?}", res);
}
