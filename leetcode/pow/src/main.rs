struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }
        if x == 1.0 {
            return 1.0;
        }
        if n == 0 {
            return 1.0;
        }
        if n == 1 {
            return x;
        }
        if n == i32::MIN {
            return 1.0 / (Solution::my_pow(x, i32::MAX) * x);
        }
        if n < 0 {
            return 1.0 / Solution::my_pow(x, -n);
        }

        let mut result_a: f64 = x;
        let mut result_b: f64 = 1.0;
        let mut base = x;
        let mut exp = n;
        while exp > 1 {
            // println!("Result is {}, exp is {}", result, exp);
            if exp % 2 == 0 {
                result_a *= result_a;
                base = result_a;
                exp /= 2;
            } else {
                result_b *= base;
                exp -= 1;
            }
        }

        result_a * result_b
    }
}

fn main() {
    println!("{}", Solution::my_pow(2.0, -2147483648));
}
