use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut window_start_index: usize = 0;
        let mut current_window_size: usize = 0;
        let mut positions: HashMap<char, usize> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            let found: Option<usize>;
            match positions.get(&c) {
                Some(old_index) => found = Some(*old_index),
                None => found = None,
            }
            match found {
                Some(old_index) => {
                    if current_window_size > longest {
                        longest = current_window_size;
                    }

                    for j in window_start_index..=old_index {
                        positions.remove(&s.chars().nth(j).unwrap());
                    }
                    positions.insert(c, i);
                    window_start_index = old_index + 1;
                    current_window_size = i - window_start_index + 1;
                }
                None => {
                    positions.insert(c, i);
                    current_window_size += 1;
                }
            }
        }

        if current_window_size > longest {
            return current_window_size as i32;
        } else {
            return longest as i32;
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("pwwkew"))
    );
}
