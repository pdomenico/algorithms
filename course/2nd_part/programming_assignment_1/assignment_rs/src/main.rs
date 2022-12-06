// read first 10 lines from edges.txt and print them out
use std::fs::File;
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug)]
struct Node {
    value: i32,
    edges_to: Vec<i32>,
}

struct SCC {
    size: i32,
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
        graph_rev.push(Node {
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
    let mut first_pass: Vec<usize> = Vec::new();
    let mut sccs: Vec<SCC> = Vec::new();

    // Run the DFSs with increased stack size
    thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let mut visited = [false; 875714];
            for node in &graph_rev {
                if !visited[(node.value - 1) as usize] {
                    first_dfs(
                        &graph_rev,
                        &mut first_pass,
                        &mut visited,
                        node.value as usize,
                    );
                }
            }

            let mut visited = [false; 875714];
            for node in first_pass.iter().rev() {
                if visited[(*node) - 1] {
                    continue;
                }

                let mut new_scc = SCC { size: 0 };

                second_dfs(&graph, &mut new_scc, &mut visited, *node);
                sccs.push(new_scc);
            }

            // sort sccs
            sccs.sort_by(|a, b| b.size.cmp(&a.size));
            for i in 0..5 {
                println!("{}", sccs[i].size);
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

fn first_dfs(graph: &Vec<Node>, first_pass: &mut Vec<usize>, visited: &mut [bool], node: usize) {
    if visited[node - 1] {
        return;
    }
    visited[node - 1] = true;

    for subnode in &graph[node - 1].edges_to {
        first_dfs(graph, first_pass, visited, *subnode as usize);
    }

    first_pass.push(node);
}

fn second_dfs(graph: &Vec<Node>, scc: &mut SCC, visited: &mut [bool], node: usize) {
    if visited[node - 1] {
        return;
    }
    visited[node - 1] = true;

    for subnode in &graph[node - 1].edges_to {
        second_dfs(graph, scc, visited, *subnode as usize)
    }
    scc.size += 1;
}


