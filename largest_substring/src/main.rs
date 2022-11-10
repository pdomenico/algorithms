use core::cmp::max;
use std::{collections::HashMap, hash::Hash};
struct Solution {}

impl Solution {
    fn find_longest(s: &mut String) -> (i32, i32, i32) {
        let len = s.len();
        if len == 1 {
            return (1, 0, 0);
        }

        let mut left_half = s;
        let mut right_half = &left_half.split_off((len / 2) + 1);
        let left_half_length = left_half.len() as i32;
        let right_half_length = right_half.len() as i32;
        let (left_max_length, left_begin, left_end) = Self::find_longest(&mut left_half);
        let (right_max_length, right_begin, right_end) = Self::find_longest(right_half);

        let mut positions: HashMap<char, i32> = HashMap::new();
        // let left_chars = left_half.chars();
        // let right_chars = right_half.chars();

        let mut starting_index_overall: i32;
        let mut ending_index_overall: i32;
        let mut max_length_overall: i32;
        if left_max_length > right_max_length {
            starting_index_overall = left_begin;
            ending_index_overall = left_end;
            max_length_overall = left_max_length;
        } else {
            starting_index_overall = right_begin;
            ending_index_overall = right_end;
            max_length_overall = right_max_length;
        }

        let mut split_length = 0;
        let mut starting_index: i32;
        // if the longest substring from the left half does not end at the end of the left half, try to find a longer subs starting from 1 char after
        if left_end < left_half_length - 1 {
            let mut i = (left_end + 1);
            starting_index = i;
            while i < (left_half_length + right_end + 1) {
                if i < left_half_length {
                    let character = left_half.chars().nth(i as usize).unwrap();
                    match positions.get(&character) {
                        Some(v) => {
                            if split_length > max_length_overall {
                                max_length_overall = split_length;
                                starting_index_overall = starting_index;
                                ending_index_overall = i - 1;
                            }
                            i = *v;
                            starting_index = i;
                            split_length = 0;
                            positions = HashMap::new();
                        }
                        None => {
                            positions.insert(character, i);
                            split_length += 1;
                            i += 1;
                        }
                    }
                } else {
                    let character = right_half
                        .chars()
                        .nth((i - left_half_length) as usize)
                        .unwrap();
                    match positions.get(&character) {
                        Some(v) => {
                            if split_length > max_length_overall {
                                max_length_overall = split_length;
                                starting_index_overall = starting_index;
                                ending_index_overall = i - 1;
                            }
                            i = *v;
                            split_length = 0;
                            positions = HashMap::new();
                        }
                        None => {
                            positions.insert(character, i);
                            split_length += 1;
                            i += 1;
                        }
                    }
                }
            }
        } else {
            for i in left_begin..(left_end + 1) {
                positions.insert(left_half.chars().nth(i as usize).unwrap(), i);
            }
            split_length = left_max_length;
            let mut i = left_end + 1;
            while i < (left_half_length + right_end + 1) {
                let character = &right_half
                    .chars()
                    .nth((i - left_half_length) as usize)
                    .unwrap();
                match positions.get(character) {
                    Some(v) => {
                        if split_length > max_length_overall {
                            max_length_overall = split_length;
                            starting_index_overall = starting_index;
                            ending_index_overall = i - 1;
                        }
                        positions = HashMap::new();
                        split_length = 0;
                        i = *v;
                    }
                    None => {
                        positions.insert(*character, i);
                        i += 1;
                        split_length += 1;
                    }
                }
            }
        }

        return (
            max_length_overall,
            starting_index_overall,
            ending_index_overall,
        );
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let (max_length, _, _) = Self::find_longest(&mut s);
        return max_length;
    }
}
fn main() {
    let res: usize = 5 / 2;
    println!("{}", res);
}
