use core::cmp::max;
use std::{collections::HashMap, hash::Hash};
struct Solution {}

impl Solution {
    fn find_longest(s: String) -> (i32, i32, i32) {
        let len = s.len();
        if len == 1 {
            return (1, 0, 0);
        }

        let left_half = s;
        let right_half = left_half.split_off((len / 2) + 1);
        let left_half_length = left_half.len() as i32;
        let right_half_length = right_half.len() as i32;
        let (left_max_length, left_begin, left_end) = Self::find_longest(left_half);
        let (right_max_length, right_begin, right_end) = Self::find_longest(right_half);

        let mut positions: HashMap<char, i32> = HashMap::new();
        let left_chars = left_half.chars();
        let right_chars = right_half.chars();

        let mut split_length = 0;
        if left_end < left_half_length {
            let i = (left_end + 1);
            while i < (left_half_length + right_half_length) {
                if i < left_half_length {
                    match positions.get(&left_chars.nth(i as usize).unwrap()) {
                        Some(v) => {
                            i = *v;
                            split_length = 0;
                            positions = HashMap::new();
                        }
                        None => {
                            positions.insert(left_chars.nth(i as usize).unwrap(), i);
                            split_length += 1;
                            i += 1;
                        }
                    }
                } else {
                    match positions.get(&right_chars.nth((i - left_half_length) as usize).unwrap())
                    {
                        Some(v) => {
                            i = *v;
                            split_length = 0;
                            positions = HashMap::new();
                        }
                        None => {
                            positions.insert(
                                right_chars.nth((i - left_half_length) as usize).unwrap(),
                                i,
                            );
                            split_length += 1;
                            i += 1;
                        }
                    }
                }
            }
        }
        for i in left_begin..left_end + 1 {
            positions.insert(left_chars.nth(i), i);
        }

        return (0, 0, 0);
    }

    pub fn length_of_longest_substring(s: String) -> i32 {}
}
fn main() {
    let res: usize = 5 / 2;
    println!("{}", res);
}
