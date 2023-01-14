use crate::union_find::UnionFind;
use core::panic;
use fxhash::FxHashMap;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::fs::File;
use std::{io, io::BufRead};
type Node = [u8; 3];

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

    let mut nodes = FxHashMap::default();
    let mut i = 0;

    while let Some(Ok(line)) = reader.next() {
        let mut bits = line.split_whitespace();
        let mut node: Node = [0; 3];
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

    let mut clusters: UnionFind = UnionFind::new(nodes_n);
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

        for i in 0..3 {
            for j in 0..8 {
                let mut one_flipped = node.clone();
                one_flipped[i] ^= 1 << (7 - j);

                if let Some(index) = nodes.get(&one_flipped) {
                    clusters.union(*node_idx as usize, *index).unwrap();
                }

                for i2 in 0..3 {
                    for j2 in 0..8 {
                        if i == i2 && j == j2 {
                            continue;
                        }

                        let mut two_flipped = one_flipped.clone();
                        two_flipped[i2] ^= 1 << (7 - j2);

                        if let Some(index) = nodes.get(&two_flipped) {
                            clusters.union(*node_idx as usize, *index).unwrap();
                        }
                    }
                }
            }
        }
        processed_nodes += 1;
    }

    let mut clusters_hashset = HashSet::new();
    for (_, node_idx) in nodes.iter() {
        // println!("node: {}", node_idx);
        clusters_hashset.insert(clusters.find(*node_idx as usize).unwrap());
    }
    println!("{}", clusters_hashset.len());
}
