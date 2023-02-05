use assignment1::floyd_warshall;
use assignment1::johnson;
use std::fs::File;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const STACK_SIZE: usize = 1024 * 1024 * 1024;

fn read_input(filenames: &Vec<&str>) -> Vec<(Vec<johnson::Edge>, usize)> {
    let files = filenames
        .iter()
        .map(|name| File::open(name).unwrap())
        .collect::<Vec<_>>();

    let mut readers = files
        .into_iter()
        .map(|file| std::io::BufReader::new(file).lines())
        .collect::<Vec<_>>();

    let n_vertices = readers
        .iter_mut()
        .map(|reader| {
            reader
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let graphs = readers
        .iter_mut()
        .map(|reader| {
            let mut graph = Vec::new();
            while let Some(Ok(line)) = reader.next() {
                let mut line = line.split_whitespace();
                graph.push(johnson::Edge {
                    tail: line.next().unwrap().parse().unwrap(),
                    head: line.next().unwrap().parse().unwrap(),
                    weight: line.next().unwrap().parse().unwrap(),
                })
            }
            graph
        })
        .collect::<Vec<_>>();

    graphs.into_iter().zip(n_vertices.into_iter()).collect()
}

fn with_floyd_warshall(graphs: &Vec<Vec<johnson::Edge>>, n_vertices: &Vec<usize>) {
    println!("Floyd-Warshall...");
    let graphs = graphs.clone();
    let n_vertices = n_vertices.clone();
    let shortest_paths_matrices = Arc::new(Mutex::new(Vec::new()));
    let shortest_paths_clone = shortest_paths_matrices.clone();
    let n_vertices_clone = n_vertices.clone();
    let new_thread = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            let mut paths = shortest_paths_clone.lock().unwrap();
            for ((i, graph), n) in graphs.iter().enumerate().zip(n_vertices_clone.iter()) {
                println!("Processing graph {}", i + 1);
                paths.push(floyd_warshall::floyd_warshall(graph, *n));
            }
            let mut shortest_paths = Vec::new();
            'outer: for (i, shortest_path_matrix) in paths.iter().enumerate() {
                for j in 1..=n_vertices[i] {
                    for k in 1..=n_vertices[i] {
                        if let Some(matrix) = shortest_path_matrix {
                            if let Some(path) = matrix[j][k] {
                                shortest_paths.push(path);
                            }
                        } else {
                            println!("Negative cost cycle detected in graph {}", i + 1);
                            continue 'outer;
                        }
                    }
                }
            }

            println!(
                "Shortest path overall with floyd-warshall: {}",
                shortest_paths.iter().min().unwrap()
            );
        })
        .unwrap();
    new_thread.join().unwrap();
}

fn with_johnson(graphs: &Vec<Vec<johnson::Edge>>, n_vertices: &Vec<usize>) {
    println!("Johnson...");
    let mut total_paths = Vec::new();

    for (i, (graph, n_vertices)) in graphs.iter().zip(n_vertices.iter()).enumerate() {
        if let Some(paths) = johnson::johnson(graph, *n_vertices) {
            paths.iter().for_each(|p| {
                p.iter().for_each(|path| {
                    if let Some(path) = path {
                        total_paths.push(*path)
                    }
                })
            })
        } else {
            println!("Negative cost cycle on graph {}!", i + 1);
        }
    }

    println!(
        "Shortest path overall with Johnson: {}",
        total_paths.iter().min().unwrap()
    );
}

fn main() {
    let graphs_with_size = read_input(&vec!["g1.txt", "g2.txt", "g3.txt"]);
    let (mut graphs, mut n_vertices) = (vec![], vec![]);
    for (graph, n) in graphs_with_size {
        graphs.push(graph);
        n_vertices.push(n);
    }

    let start = Instant::now();
    with_floyd_warshall(&graphs, &n_vertices);
    println!("Floyd-Warshall took {:?}\n", start.elapsed());
    let start = Instant::now();
    with_johnson(&graphs, &n_vertices);
    println!("Johnson took {:?}", start.elapsed());
}
