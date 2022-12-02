// read first 10 lines from edges.txt and print them out
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    value: i32,
    edges_to: Vec<i32>,
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut graph: Vec<Node> = Vec::with_capacity(875714);
    let mut graph_rev: Vec<Node> = Vec::with_capacity(875714);

    for i in 1..=875714 {
        graph.push(Node {
            value: i,
            edges_to: Vec::new(),
        });
        graph.push(Node {
            value: i,
            edges_to: Vec::new(),
        });
    }

    if let Ok(lines) = read_lines("edges.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut iter = line.split_whitespace();
                let tail = iter
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Error with file");
                let head = iter
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Error with file");

                graph[tail - 1].edges_to.push(head as i32);
                graph_rev[head - 1].edges_to.push(tail as i32);
            }
        }
    }

    for i in 0..3 {
        print!("{:?}", graph[i]);
    }
}
