struct Solution {

}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s == String::from("") {
            return 0;
        }

        let mut longest_length: usize = 1;
        let mut longest: Vec<char> = Vec::new();

        for c in s.chars() {
            if longest.contains(&c) {
                if longest.len() > longest_length {
                    longest_length = longest.len();
                }
                longest = vec![c];
                continue;
            }
            longest.push(c);
        }

        return max(longest_length, longest.len());
    }
}

fn main() {
    println!("{}", Solution::length_of_longest_substring(String::from("au")));
}