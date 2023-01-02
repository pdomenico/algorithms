use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Edge {
    vec_a: i32,
    vec_b: i32,
    weight: i32,
}

fn main() {
    let filename = "edges.txt";
    let mut edges = Vec::new();
    let mut vectors = HashMap::new();
    let mut vec_n = 0;

    let file = File::open(filename).expect("Could not open file!");
    let lines = BufReader::new(file).lines();
    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split_whitespace().collect();
        if i == 0 {
            vec_n = values[0].parse().unwrap();
            continue;
        }
        let vec_a = values[0].parse::<i32>().unwrap();
        let vec_b = values[1].parse::<i32>().unwrap();
        let weight = values[2].parse::<i32>().unwrap();
        edges.push(Edge {
            vec_a,
            vec_b,
            weight,
        });
        vectors.insert(vec_a, false);
        vectors.insert(vec_b, false);
    }

    let mut mst = Vec::new();
    vectors.insert(1, true);
    vec_n -= 1;
    while vec_n > 0 {
        let mut shortest_edge = &Edge {
            vec_a: 0,
            vec_b: 0,
            weight: i32::MAX,
        };

        for edge in &edges {
            if let (Some(bool_a), Some(bool_b)) =
                (vectors.get(&edge.vec_a), vectors.get(&edge.vec_b))
            {
                if !(bool_a ^ bool_b) {
                    continue;
                }

                if edge.weight < shortest_edge.weight {
                    shortest_edge = edge;
                }
            }
        }

        vectors.insert(shortest_edge.vec_a, true);
        vectors.insert(shortest_edge.vec_b, true);
        vec_n -= 1;
        mst.push(shortest_edge);
    }

    let mut total_weight: i64 = 0;
    for edge in mst {
        total_weight += edge.weight as i64;
    }
    println!("{total_weight}");
}
