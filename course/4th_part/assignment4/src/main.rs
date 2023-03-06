use std::{collections::HashMap, fs::File, io, io::BufRead};

#[derive(Clone, Debug)]
struct Vertex {
    outgoing_edges: Vec<usize>,
    incoming_edges: Vec<usize>,
}

impl Vertex {
    fn new() -> Self {
        Vertex {
            outgoing_edges: Vec::new(),
            incoming_edges: Vec::new(),
        }
    }
}

fn add_edge(graph: &mut Vec<Vertex>, from: usize, to: usize) {
    graph[from].outgoing_edges.push(to);
    graph[to].incoming_edges.push(from);
}

fn read_inputs() -> Vec<(usize, Vec<(i32, i32)>)> {
    let mut ret = Vec::new();
    for i in 1..=6 {
        let mut constraints = Vec::new();
        let mut filename = String::from("2sat");
        filename.push_str(&i.to_string());
        filename.push_str(".txt");
        let file = File::open(filename).unwrap();
        let mut reader = io::BufReader::new(file).lines();
        let variables_n = reader.next().unwrap().unwrap().parse::<usize>().unwrap();

        while let Some(Ok(line)) = reader.next() {
            constraints.push((
                line.split_whitespace()
                    .nth(0)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
                line.split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            ));
        }
        ret.push((variables_n, constraints));
    }
    ret
}

fn read_test_input() -> (usize, Vec<(i32, i32)>) {
    let file = File::open("test.txt").unwrap();
    let mut reader = io::BufReader::new(file).lines();
    let variables_n = reader.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut constraints = Vec::new();
    while let Some(Ok(line)) = reader.next() {
        constraints.push((
            line.split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            line.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        ));
    }
    (variables_n, constraints)
}

fn generate_graph(variables_n: usize, constraints: &Vec<(i32, i32)>) -> Vec<Vertex> {
    let mut graph = vec![Vertex::new(); variables_n * 2 + 1];

    for (val1, val2) in constraints {
        let vert1 = if *val1 < 0 {
            -val1 + variables_n as i32
        } else {
            *val1
        };
        let vert1_inv = if vert1 <= variables_n as i32 {
            vert1 + variables_n as i32
        } else {
            vert1 - variables_n as i32
        };
        let vert2 = if *val2 < 0 {
            -val2 + variables_n as i32
        } else {
            *val2
        };
        let vert2_inv = if vert2 <= variables_n as i32 {
            vert2 + variables_n as i32
        } else {
            vert2 - variables_n as i32
        };
        add_edge(&mut graph, vert1_inv as usize, vert2 as usize);
        add_edge(&mut graph, vert2_inv as usize, vert1 as usize);
    }
    graph
}

fn reverse_graph(graph: &Vec<Vertex>) -> Vec<Vertex> {
    let mut output = Vec::new();
    for vertex in graph {
        output.push(Vertex {
            incoming_edges: vertex.outgoing_edges.clone(),
            outgoing_edges: vertex.incoming_edges.clone(),
        });
    }
    output
}

fn dfs(graph: &Vec<Vertex>, from: usize, queue: &mut Vec<usize>, visited: &mut Vec<bool>) {
    visited[from] = true;
    for vertex in graph[from].outgoing_edges.iter() {
        if !visited[*vertex] {
            dfs(graph, *vertex, queue, visited);
        }
    }

    queue.push(from);
}

fn vec_n_to_variable(vec_n: usize, variables_n: usize) -> (usize, bool) {
    if vec_n > variables_n {
        (vec_n - variables_n, false)
    } else {
        (vec_n, true)
    }
}

fn check_validity(sccs: &Vec<Vec<usize>>, variables_n: usize) -> bool {
    for scc in sccs {
        let mut variables = vec![false; variables_n + 1];
        for val in scc {
            let val = if *val > variables_n {
                val - variables_n
            } else {
                *val
            };
            if variables[val] {
                return false;
            }
            variables[val] = true;
        }
    }
    true
}

fn run_test() {
    let (variables_n, constraints) = read_test_input();
    let graph = generate_graph(variables_n, &constraints);

    let mut queue = Vec::new();
    let mut visited = vec![false; variables_n * 2 + 1];

    for i in 1..graph.len() {
        if visited[i] {
            continue;
        };
        dfs(&graph, i, &mut queue, &mut visited);
    }

    let reversed_graph = reverse_graph(&graph);
    queue.reverse();

    let mut visited = vec![false; variables_n * 2 + 1];
    let mut sccs = Vec::new();

    for start in queue {
        if visited[start] {
            continue;
        }
        let mut scc = Vec::new();
        dfs(&reversed_graph, start, &mut scc, &mut visited);
        sccs.push(scc);
    }
    // println!("Sccs: {:?}", sccs);

    match check_validity(&sccs, variables_n) {
        true => println!("Valid 2SAT!"),
        false => println!("Invalid 2SAT!"),
    }
}

fn run() {
    let inputs = read_inputs();
    let mut results = Vec::new();

    for (i, (variables_n, constraints)) in inputs.into_iter().enumerate() {
        println!("Analyzing SAT number {}", i + 1);

        let graph = generate_graph(variables_n, &constraints);
        let mut queue = Vec::new();
        let mut visited = vec![false; variables_n * 2 + 1];

        println!("First dfs round...");
        for i in 1..graph.len() {
            if visited[i] {
                continue;
            }
            dfs(&graph, i, &mut queue, &mut visited);
        }
        queue.reverse();

        let reversed_graph = reverse_graph(&graph);
        let mut visited = vec![false; variables_n * 2 + 1];
        let mut sccs = Vec::new();

        println!("Second dfs round...");
        for start in queue {
            if visited[start] {
                continue;
            }
            let mut scc = Vec::new();
            dfs(&reversed_graph, start, &mut scc, &mut visited);
            sccs.push(scc);
        }

        println!("Checking validity...");
        results.push(check_validity(&sccs, variables_n));
    }

    for result in results {
        match result {
            true => print!("1"),
            false => print!("0"),
        }
    }
    println!();
}

fn main() {
    run();
}
