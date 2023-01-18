use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut input = io::BufReader::new(file).lines();
    let vertices_n: usize = input.next().unwrap().unwrap().parse().unwrap();
    let mut weights = Vec::with_capacity(vertices_n);

    while let Some(Ok(line)) = input.next() {
        weights.push(line.parse::<i64>().unwrap());
    }

    let mut result = vec![1; vertices_n];

    for i in 0..vertices_n {
        if i == 0 {
            result[i] = weights[i];
            continue;
        }
        if i == 1 {
            result[i] = std::cmp::max(weights[0], weights[1]);
            continue;
        }

        result[i] = std::cmp::max(result[i - 1], result[i - 2] + weights[i]);
    }

    let mut solution = vec![false; vertices_n];

    let mut i = vertices_n - 1;
    while i >= 1 {
        if i == 1 {
            if result[0] > weights[i] {
                i -= 1;
            } else {
                solution[i] = true;
            }
        } else {
            if result[i - 1] > result[i - 2] + weights[i] {
                i -= 1;
            } else {
                solution[i] = true;
                i -= 2;
            }
        }
    }

    if !solution[1] {
        solution[0] = true;
    }

    for i in [1, 2, 3, 4, 17, 117, 517, 997] {
        match solution[i - 1] {
            true => print!("1"),
            false => print!("0"),
        }
    }
    println!();

    // for (i, sol) in solution.iter().enumerate() {
    //     println!("{}: {}", i + 1, sol);
    // }
    // println!("Max weight: {}", result[vertices_n - 1]);
}
