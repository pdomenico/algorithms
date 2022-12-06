use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Edge {
    head: usize,
    length: i32,
}

type Vertex = Vec<Edge>;

fn main() {
    let mut graph: Vec<Vertex> = Vec::new();

    let file = File::open("edges.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let data: Vec<&str> = line.split_whitespace().collect();
        let mut last_vertex = 0;
        for (i, data_point) in data.iter().enumerate() {
            if i == 0 {
                graph.push(Vec::new());
                last_vertex = data_point.parse().unwrap();
                continue;
            }

            let data_point: Vec<&str> = data_point.split(',').collect();
            graph[last_vertex - 1].push(Edge {
                head: data_point[0].parse().unwrap(),
                length: data_point[1].parse().unwrap(),
            })
        }
    }

    let mut shortest_paths: HashMap<usize, i32> = HashMap::new();
    shortest_paths.insert(0, 0);

    loop {
        let mut candidates: Vec<(usize, i32)> = Vec::new();
        for (vx, sp) in &shortest_paths {
            for outgoing_edge in &graph[*vx] {
                if shortest_paths.contains_key(&(outgoing_edge.head - 1)) {
                    continue;
                }
                candidates.push((outgoing_edge.head - 1, *sp + outgoing_edge.length));
            }
        }

        if candidates.len() == 0 {
            break;
        }

        let mut shortest = 0;
        let mut shortest_head: usize = 0;
        for (c_vert, c_length) in &candidates {
            if *c_length < shortest || shortest == 0 {
                shortest = *c_length;
                shortest_head = *c_vert;
            }
        }
        shortest_paths.insert(shortest_head, shortest);
    }

    let destinations: Vec<usize> = vec![7, 37, 59, 82, 99, 115, 133, 165, 188, 197];
    for des in destinations {
        print!("{},", shortest_paths[&(des - 1)]);
    }
}
