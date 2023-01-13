use core::panic;
use disjoint_sets::UnionFind;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::{io, io::BufRead};
type Node = [u8; 3];

pub fn main_with_union_find() {
    let file = File::open("input2.txt").unwrap();
    let mut reader = io::BufReader::new(file).lines();
    let nodes_n: usize = reader
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut nodes = HashMap::with_capacity(nodes_n);
    let mut i = 0;

    while let Some(Ok(line)) = reader.next() {
        let mut bits = line.split_whitespace();
        let mut node: [u8; 3] = [0; 3];
        for i in 0..3 {
            for j in 0..8 {
                match bits.next() {
                    Some("1") => node[i] ^= 1 << (7 - j),
                    Some("0") => (),
                    _ => {
                        println!("{line}");
                        panic!("invalid input");
                    }
                }
            }
        }
        nodes.insert(node, i);
        i += 1;
    }

    let mut clusters: UnionFind<usize> = UnionFind::new(nodes_n);
    let mut processed_nodes = 0;

    let progress_bar = ProgressBar::new(nodes_n as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .expect("")
            .progress_chars("=> "),
    );

    for (node, node_idx) in nodes.iter() {
        progress_bar.set_position(processed_nodes);
        // find similar nodes
        let mut similar_nodes = Vec::new();

        for i in 0..3 {
            for j in 0..8 {
                let mut one_flipped = node.clone();
                one_flipped[i] ^= 1 << (7 - j);

                if let Some(index) = nodes.get(&one_flipped) {
                    similar_nodes.push(index);
                }

                for i2 in 0..3 {
                    for j2 in 0..8 {
                        if i == i2 && j == j2 {
                            continue;
                        }

                        let mut two_flipped = one_flipped.clone();
                        two_flipped[i2] ^= 1 << (7 - j2);

                        if let Some(index) = nodes.get(&two_flipped) {
                            similar_nodes.push(index);
                        }
                    }
                }
            }
        }
        for similar_node_idx in similar_nodes {
            clusters.union(*node_idx as usize, *similar_node_idx as usize);
        }
        processed_nodes += 1;
    }

    let mut clusters_hashset = HashSet::new();
    for (_, node_idx) in nodes.iter() {
        // println!("node: {}", node_idx);
        clusters_hashset.insert(clusters.find(*node_idx as usize));
    }
    println!("{}", clusters_hashset.len());
}

pub fn main() {
    let file = File::open("input2.txt").unwrap();
    let mut reader = io::BufReader::new(file).lines();
    let nodes_n: usize = reader
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut nodes = HashSet::with_capacity(nodes_n);

    while let Some(Ok(line)) = reader.next() {
        let mut bits = line.split_whitespace();
        let mut node: [u8; 3] = [0; 3];
        for i in 0..3 {
            for j in 0..8 {
                match bits.next() {
                    Some("1") => node[i] ^= 1 << (7 - j),
                    Some("0") => (),
                    _ => {
                        println!("{line}");
                        panic!("invalid input");
                    }
                }
            }
        }
        nodes.insert(node);
    }

    let mut nodes_iter = nodes.iter();

    let mut clusters: Vec<HashSet<Node>> = Vec::new();
    let mut processed_nodes = 0;

    let progress_bar = ProgressBar::new(nodes_n as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .expect("")
            .progress_chars("=> "),
    );

    while let Some(node) = nodes_iter.next() {
        progress_bar.set_position(processed_nodes);
        let mut current_cluster_idx = clusters
            .iter()
            .position(|clust| clust.contains(node))
            .unwrap_or_else(|| {
                let mut new_cluster = HashSet::new();
                new_cluster.insert(node.clone());
                clusters.push(new_cluster);
                clusters.len() - 1
            });
        // println!("Current cluster: {:?}", clusters[current_cluster_idx]);

        // find all the other nodes that differ by 1 or 2 bits
        let mut similar_nodes = Vec::new();
        for i in 0..3 {
            for j in 0..8 {
                let mut one_flipped = node.clone();

                // search 1 bit difference
                one_flipped[i] ^= 1 << (7 - j);

                if let Some(node) = nodes.get(&one_flipped) {
                    similar_nodes.push(node);
                }

                for i2 in 0..3 {
                    for j2 in 0..8 {
                        if i == i2 && j == j2 {
                            continue;
                        }
                        let mut two_flipped = one_flipped.clone();
                        two_flipped[i2] ^= 1 << (7 - j2);

                        if let Some(node) = nodes.get(&two_flipped) {
                            similar_nodes.push(node);
                        }
                    }
                }
            }
        }

        for similar_node in similar_nodes {
            let current_cluster_idx = clusters
                .iter()
                .position(|cluster| cluster.contains(node))
                .unwrap();
            match clusters.iter().position(|c| c.contains(similar_node)) {
                None => {
                    clusters[current_cluster_idx].insert(similar_node.clone());
                }

                Some(other_cluster_idx) => {
                    if other_cluster_idx == current_cluster_idx {
                        continue;
                    }
                    let (current_clust_len, other_clust_len) = (
                        clusters[current_cluster_idx].len(),
                        clusters[other_cluster_idx].len(),
                    );

                    let (from_clust_idx, to_clust_idx) =
                        match current_clust_len.cmp(&other_clust_len) {
                            Ordering::Greater => (other_cluster_idx, current_cluster_idx),
                            _ => (current_cluster_idx, other_cluster_idx),
                        };

                    // move all nodes from the smaller cluster to the bigger one
                    let nodes_to_move = clusters[from_clust_idx].drain().collect::<Vec<_>>();
                    for node in nodes_to_move {
                        clusters[to_clust_idx].insert(node);
                    }
                    clusters.swap_remove(from_clust_idx);
                }
            }
        }
        processed_nodes += 1;
    }
    println!("Clusters: {}", clusters.len());
}

fn print_node(node: &Node) {
    println!(
        "{}{}{}",
        format!("{:08b}", node[0]),
        format!("{:08b}", node[1]),
        format!("{:08b}", node[2])
    );
}
