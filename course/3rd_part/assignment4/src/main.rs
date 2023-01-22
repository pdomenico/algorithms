use indicatif::ProgressBar;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{stdout, Write};

fn main() {
    knapsack2();
}

fn get_data(filename: &str) -> (usize, Vec<usize>, Vec<usize>) {
    let file = File::open(filename).unwrap();
    let mut reader = io::BufReader::new(file).lines();

    let first_line = reader.next().unwrap().unwrap();
    let data = first_line.split_whitespace().collect::<Vec<_>>();
    let (size, items_n): (usize, usize) = (data[0].parse().unwrap(), data[1].parse().unwrap());

    let (mut values, mut weights) = (Vec::with_capacity(items_n), Vec::with_capacity(items_n));

    println!("Reading data...");
    let pb = ProgressBar::new(values.len() as u64);
    while let Some(Ok(line)) = reader.next() {
        pb.inc(1);
        let (value, weight) = {
            let data = line.split_whitespace().collect::<Vec<_>>();
            (
                data[0].parse::<usize>().unwrap(),
                data[1].parse::<usize>().unwrap(),
            )
        };

        values.push(value);
        weights.push(weight);
    }
    (size, values, weights)
}

fn knapsack1() {
    let (size, values, weights) = get_data("input1_2.txt");
    println!("Data fully read");

    let min_weight = *weights.iter().min().unwrap();

    let mut solution = vec![vec![0usize; size + 1]; values.len()];

    println!("Calculating...");
    let pb = ProgressBar::new(values.len() as u64);

    for i in 1..values.len() {
        pb.inc(1);
        for s in min_weight..=size {
            let current_weight = weights[i];
            if current_weight > s {
                solution[i][s] = solution[i - 1][s];
            } else {
                solution[i][s] = std::cmp::max(
                    solution[i - 1][s],
                    solution[i - 1][s - current_weight] + values[i],
                );
            }
        }
    }

    println!("{}", solution[values.len() - 1][size]);
}

fn knapsack2() {
    let (capacity, values, weights) = get_data("input2.txt");
    let min_weight = weights.iter().min().unwrap().to_owned();

    let mut cached_sol = vec![HashMap::new(); values.len() + 1];

    let sol = compute_sol(
        &values,
        &weights,
        min_weight,
        &mut cached_sol,
        capacity,
        values.len(),
    );

    println!("{}", sol);
}

fn compute_sol(
    values: &Vec<usize>,
    weights: &Vec<usize>,
    min_weight: usize,
    cached_sol: &mut Vec<HashMap<usize, usize>>,
    capacity: usize,
    items: usize,
) -> usize {
    if capacity < min_weight || items == 0 {
        return 0;
    }

    if let Some(sol) = cached_sol[items].get(&capacity) {
        return *sol;
    }

    let case1 = compute_sol(values, weights, min_weight, cached_sol, capacity, items - 1);
    if weights[items - 1] > capacity {
        cached_sol[items].insert(capacity, case1);
        return case1;
    }

    let case2 = compute_sol(
        values,
        weights,
        min_weight,
        cached_sol,
        capacity - weights[items - 1],
        items - 1,
    ) + values[items - 1];

    let solution = usize::max(case1, case2);
    cached_sol[items].insert(capacity, solution);
    solution
}
