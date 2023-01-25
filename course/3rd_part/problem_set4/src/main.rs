fn main() {
    let values = vec![1, 2, 3, 4, 5, 6, 7];
    let frequencies = vec![0.20, 0.05, 0.17, 0.10, 0.20, 0.03, 0.25];
    let mut sol = vec![vec![0.0; values.len()]; values.len()];

    for s in 0..values.len() {
        for i in 0..values.len() {
            if i + s >= values.len() {
                continue;
            }
            if s == 0 {
                sol[i][i] = frequencies[i];
                continue;
            }

            let mut possibilities = Vec::new();
            for root in i..=(i + s) {
                let base = {
                    let mut sum = 0.0;
                    for j in i..=(i + s) {
                        sum += frequencies[j];
                    }
                    sum
                };
                if root == i {
                    possibilities.push(
                        base + match sol.get(i + 1) {
                            Some(v) => v[i + s],
                            None => 0.0,
                        },
                    );
                } else if root == i + s {
                    possibilities.push(base + sol[i].get(root - 1).unwrap_or_else(|| &0.0));
                } else {
                    possibilities.push(base + sol[i][root - 1] + sol[root + 1][i + s]);
                }
            }
            sol[i][i + s] = possibilities
                .into_iter()
                .fold(f64::MAX, |acc, x| f64::min(acc, x));
        }
    }

    println!("{}", sol[0][6]);
}
