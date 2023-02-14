use std::io::{stdout, Write};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

#[derive(Clone)]
pub struct Edge {
    pub tail: usize,
    pub head: usize,
    pub weight: i32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

pub fn bellman_ford(
    graph: &Vec<Edge>,
    n_vertices: usize,
    source: usize,
) -> Option<Vec<Option<i32>>> {
    let mut prev_paths: Vec<Option<i32>> = vec![None; n_vertices + 1];
    prev_paths[source] = Some(0);
    let mut new_paths: Vec<Option<i32>> = vec![None; n_vertices + 1];
    let mut head_to_tails = HashMap::new();
    let mut vecs_to_edge = HashMap::new();

    for edge in graph {
        if edge.tail == source {
            prev_paths[edge.head] = Some(edge.weight);
        }

        match head_to_tails.get_mut(&edge.head) {
            None => {
                head_to_tails.insert(edge.head, vec![edge.tail]);
            }
            Some(tails) => tails.push(edge.tail),
        }

        match vecs_to_edge.get(&(edge.tail, edge.head)) {
            Some(other_edge) => {
                if *other_edge > edge.weight {
                    vecs_to_edge.insert((edge.tail, edge.head), edge.weight);
                }
            }
            None => {
                vecs_to_edge.insert((edge.tail, edge.head), edge.weight);
            }
        }
    }

    for budget in 1..=(n_vertices + 1) {
        print!("\ri: {budget}");
        stdout().flush().unwrap();
        for destination in 1..=n_vertices {
            let case1 = prev_paths[destination];
            let case2 = {
                match head_to_tails.get(&destination) {
                    None => None,
                    Some(tails) => {
                        let mut candidates = Vec::new();
                        for tail in tails {
                            match (prev_paths[*tail], vecs_to_edge.get(&(*tail, destination))) {
                                (Some(p1), Some(p2)) => match p1.checked_add(*p2) {
                                    Some(sum) => candidates.push(sum),
                                    None => return None,
                                },
                                _ => (),
                            }
                        }
                        candidates.into_iter().min()
                    }
                }
            };
            new_paths[destination] = match (case1, case2) {
                (Some(case1), Some(case2)) => Some(case1.min(case2)),
                (Some(case1), None) => Some(case1),
                (None, Some(case2)) => Some(case2),
                (None, None) => None,
            }
        }
        if !(budget == n_vertices + 1) {
            prev_paths = new_paths;
            new_paths = vec![None; n_vertices + 1];
        }
    }
    println!();

    for destination in 1..=n_vertices {
        if new_paths[destination] != prev_paths[destination] {
            return None;
        }
    }

    Some(new_paths)
}

pub fn johnson(graph: &Vec<Edge>, n_vertices: usize) -> Option<Vec<Vec<Option<i32>>>> {
    // Add new edges to the graph
    let mut modified_graph = graph.clone();
    for i in 1..=n_vertices {
        modified_graph.push(Edge {
            tail: n_vertices + 1,
            head: i,
            weight: 0,
        });
    }

    // Run bellman-ford and get the correctors
    println!("Running Bellman-Ford...");
    let mut correctors = vec![0; n_vertices + 1];
    match bellman_ford(&modified_graph, n_vertices + 1, n_vertices + 1) {
        None => {
            // println!("Negative cycle detected!");
            return None;
        }
        Some(paths) => {
            for i in 1..=n_vertices {
                if let Some(path) = paths[i] {
                    correctors[i] = path;
                } else {
                    println!("Path not found for vertex {i}");
                }
            }
        }
    };

    let mut corrected_graph = Vec::new();
    for edge in graph {
        corrected_graph.push(Edge {
            tail: edge.tail,
            head: edge.head,
            weight: edge.weight + correctors[edge.tail] - correctors[edge.head],
        });
    }

    for edge in &corrected_graph {
        if edge.weight < 0 {
            println!("Error in Bellman-Ford");
            return None;
        }
    }

    // Run dijkstra n times
    println!("Running Dijkstra n times...");
    let mut ap_shortest_paths = Vec::new();
    ap_shortest_paths.push(vec![None]);
    let mut v_to_outgoing_edges = HashMap::new();
    for edge in &corrected_graph {
        match v_to_outgoing_edges.get_mut(&edge.tail) {
            None => {
                v_to_outgoing_edges.insert(edge.tail, vec![edge]);
            }
            Some(heads) => heads.push(edge),
        }
    }

    for source in 1..=n_vertices {
        print!("\rn: {source}");
        stdout().flush().unwrap();
        let mut crossing_edges = BinaryHeap::new();
        corrected_graph
            .iter()
            .filter(|e| e.tail == source)
            .for_each(|e| crossing_edges.push(Reverse(e)));

        let mut shortest_paths = vec![None; n_vertices + 1];
        shortest_paths[source] = Some(0);

        while let Some(Reverse(shortest_edge)) = crossing_edges.pop() {
            if shortest_paths[shortest_edge.head].is_some() {
                continue;
            }

            shortest_paths[shortest_edge.head] = Some(shortest_edge.weight);
            match v_to_outgoing_edges.get(&shortest_edge.head) {
                Some(outgoing_edges) => outgoing_edges
                    .iter()
                    .filter(|edge| shortest_paths[edge.head].is_none())
                    .for_each(|edge| crossing_edges.push(Reverse(edge))),
                None => (),
            }
        }
        ap_shortest_paths.push(shortest_paths);
    }
    println!();

    for source in 1..=n_vertices {
        for dest in 1..=n_vertices {
            if let Some(path) = ap_shortest_paths[source][dest] {
                ap_shortest_paths[source][dest] =
                    Some(path - correctors[source] + correctors[dest]);
            }
        }
    }

    Some(ap_shortest_paths)
}
