use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn hash(str: &str) -> u32 {
        str.trim().chars().map(|c| c as u32).sum::<u32>()
    }

    pub fn are_anagrams(s1: &String, s2: &String) -> bool {
        let mut copy2 = s2.clone();
        for c in s1.chars() {
            match copy2.find(c) {
                None => return false,
                Some(index) => copy2.remove(index),
            };
        }

        copy2.len() == 0
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let hashes: Vec<u32> = strs
            .iter()
            .map(|s| s.as_str())
            .map(|s| Solution::hash(s))
            .collect();

        let mut groups: HashMap<u32, Vec<Vec<String>>> = HashMap::new();

        strs.into_iter()
            .zip(hashes.iter())
            .for_each(|(s, h)| match groups.get_mut(h) {
                Some(vec) => {
                    let mut found = false;
                    vec.iter_mut().for_each(|group| {
                        if Solution::are_anagrams(&group[0], &s) {
                            found = true;
                            group.push(s.clone());
                        }
                    });
                    if !found {
                        vec.push(vec![s.clone()]);
                    }
                }
                None => {
                    groups.insert(*h, vec![vec![s]]);
                }
            });

        let mut sol = Vec::new();
        // hashes.iter().for_each(|h| match groups.get_mut(h) {
        //     Some(group) => {
        //         sol.append(group);
        //     }
        //     None => (),
        // });
        sol
    }
}

fn main() {}
