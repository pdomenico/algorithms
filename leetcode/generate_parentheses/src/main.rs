struct Solution {}

impl Solution {
    pub fn generate_parentheses(n: i32) -> Vec<String> {
       return vec![String::new()];
    }

    pub fn split_subparentheses(s: String) -> Vec<String> {
        let mut res: Vec<String> = vec![String::from("")];
        
        let mut n = 0;
        for (i, c) in s.chars().enumerate() {
            let last = res.len() -1;
            match c {
                '(' => {
                    res[last].push(c);
                    n += 1;
                }
                _ =>{
                    res[last].push(c);
                    n -= 1;
                }
            } 
            if n == 0 && i != s.len()-1{
                res.push(String::from(""));
            }
        }
        
        res
    }

    pub fn add_parentheses(s: String) -> Vec<String> {
        let subexps = Solution::split_subparentheses(s);
        let mut res = Vec::new();

        for (i, subexp) in subexps.iter().enumerate() {
            for possibility in Solution::add_inside_parentheses(subexp) {
                
            }
        }
    }

    pub fn add_inside_parentheses(s: String) -> Vec<String> {
        match s.as_str() {
            "()" => vec![String::from("(())")],
            _ => Solution::add_parentheses(s),
        }
    }
}

fn main()  {
    println!("{:?}", Solution::add_parentheses(String::from("()()")));
}


