use std::cmp::Ordering;
use std::fs::File;
use std::{io, io::BufRead};

#[derive(Clone)]
struct Edge {
    from: i32,
    to: i32,
    weight: i32,
}

pub fn part1() {
    let file = File::open("input.txt").unwrap();
    let mut reader = io::BufReader::new(file).lines();
    let nodes_n: usize = reader.next().unwrap().unwrap().parse().unwrap();
    let mut edges = Vec::new();
    while let Some(Ok(line)) = reader.next() {
        let mut data = line.split_whitespace();
        edges.push(Edge {
            from: data.next().unwrap().parse::<i32>().unwrap() - 1,
            to: data.next().unwrap().parse::<i32>().unwrap() - 1,
            weight: data.next().unwrap().parse::<i32>().unwrap(),
        });
    }

    edges.sort_by_key(|e| e.weight);
    edges.reverse();

    // for e in edges.iter() {
    //     println!("{} {} {}", e.from, e.to, e.weight);
    // }

    let mut clusters: Vec<Vec<i32>> = Vec::with_capacity(nodes_n);
    for i in 0..nodes_n {
        clusters.push(vec![i as i32]);
    }
    let k = 4;
    // for (i, cluster) in clusters.iter().enumerate() {
    //     println!("Cluster {i}: {cluster:?}");
    // }

    while clusters.len() > k {
        let shortest_edge = edges.pop().unwrap();

        let (from_cluster_idx, to_cluster_idx) = (
            clusters
                .iter()
                .position(|c| c.contains(&shortest_edge.from))
                .unwrap(),
            clusters
                .iter()
                .position(|c| c.contains(&shortest_edge.to))
                .unwrap(),
        );

        if from_cluster_idx == to_cluster_idx {
            continue;
        }

        // println!(
        //     "\nLongest: {} {} {}",
        //     shortest_edge.from, shortest_edge.to, shortest_edge.weight
        // );

        let (smallest_clust_idx, biggest_clust_idx) = match clusters[from_cluster_idx]
            .len()
            .cmp(&clusters[to_cluster_idx].len())
        {
            Ordering::Less => (from_cluster_idx, to_cluster_idx),
            _ => (to_cluster_idx, from_cluster_idx),
        };

        while let Some(node) = clusters[smallest_clust_idx].pop() {
            clusters[biggest_clust_idx].push(node);
        }
        clusters.remove(smallest_clust_idx);

        // for (i, cluster) in clusters.iter().enumerate() {
        //     println!("Cluster {i}: {cluster:?}");
        // }
    }

    let distance = edges
        .iter()
        .rev()
        .find(|edge| {
            let (from_cluster, to_cluster) = (
                clusters
                    .iter()
                    .position(|c| c.contains(&edge.from))
                    .unwrap(),
                clusters.iter().position(|c| c.contains(&edge.to)).unwrap(),
            );
            from_cluster != to_cluster
        })
        .unwrap()
        .weight;
    println!("{}", distance);
}
