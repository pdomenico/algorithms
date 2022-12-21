use fnv::FnvHashMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn add_to_hashmap(filename: &str, hashmap: &mut FnvHashMap<i64, bool>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let n: i64 = line.unwrap().parse().unwrap();

        hashmap.insert(n, true);
    }
}

fn main() {
    let mut hashmap: FnvHashMap<i64, bool> = FnvHashMap::default();
    let mut numbers = Vec::new();

    add_to_hashmap("numbers.txt", &mut hashmap);

    let file = File::open("numbers.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let n: i64 = line.unwrap().parse().unwrap();
        numbers.push(n);
    }

    let mut res = 0;

    'outer: for i in -10000..10001 {
        println!("Checking for {i}, res={res}");
        'inner: for x in &numbers {
            let y: i64 = i - *x;
            if y == *x {
                continue 'inner;
            }

            if let Some(_) = hashmap.get(&y) {
                res += 1;
                println!("Found one pair! {x}:{y}");
                continue 'outer;
            }
        }
    }

    println!("{res}");
}
