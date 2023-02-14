use crate::johnson::Edge;
use std::io::{stdout, Write};

const MAX_N_VERTICES: usize = 2049;

pub fn floyd_warshall(graph: &Vec<Edge>, n_vertices: usize) -> Option<Vec<Vec<Option<i32>>>> {
    let mut paths1: [[Option<i32>; MAX_N_VERTICES]; MAX_N_VERTICES] =
        [[None; MAX_N_VERTICES]; MAX_N_VERTICES];
    let mut paths2: [[Option<i32>; MAX_N_VERTICES]; MAX_N_VERTICES] =
        [[None; MAX_N_VERTICES]; MAX_N_VERTICES];

    for edge in graph {
        paths1[edge.tail][edge.head] = Some(edge.weight);
    }

    for i in 1..=n_vertices {
        paths1[i][i] = Some(0);
    }

    for k in 1..=n_vertices {
        print!("\rk: {k}");
        stdout().flush().unwrap();
        for i in 1..=n_vertices {
            for j in 1..=n_vertices {
                let case1 = paths1[i][j];

                let case2 = match (paths1[i][k], paths1[k][j]) {
                    (Some(p1), Some(p2)) => match p1.checked_add(p2) {
                        Some(sum) => Some(sum),
                        None => {
                            println!();
                            return None;
                        }
                    },
                    _ => None,
                };

                paths2[i][j] = match (case1, case2) {
                    (Some(case1), Some(case2)) => Some(case1.min(case2)),
                    (Some(case1), None) => Some(case1),
                    (None, Some(case2)) => Some(case2),
                    (None, None) => None,
                }
            }
        }
        paths1 = paths2;
        paths2 = [[None; MAX_N_VERTICES]; MAX_N_VERTICES];
    }
    println!();

    // Check for negative cost cycles
    for i in 1..=n_vertices {
        if paths1[i][i].unwrap() < 0 {
            return None;
        }
    }
    Some(
        paths1
            .into_iter()
            .map(|subp| subp.into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}
