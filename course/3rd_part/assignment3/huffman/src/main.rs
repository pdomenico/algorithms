use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

enum Node {
    Leaf(i64),
    Intermediate(Box<Node>, Box<Node>),
}

impl Node {
    fn value(&self) -> i64 {
        match self {
            Node::Leaf(v) => *v,
            Node::Intermediate(left, right) => left.value() + right.value(),
        }
    }

    fn max_height(&self) -> i64 {
        match self {
            Node::Leaf(_) => 0,
            Node::Intermediate(left, right) => {
                1 + std::cmp::max(left.max_height(), right.max_height())
            }
        }
    }

    fn min_height(&self) -> i64 {
        match self {
            Node::Leaf(_) => 0,
            Node::Intermediate(left, right) => {
                1 + std::cmp::min(left.min_height(), right.min_height())
            }
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut input = io::BufReader::new(file).lines();
    let letters_n: usize = input.next().unwrap().unwrap().parse().unwrap();
    let mut letters: BinaryHeap<Reverse<Node>> = BinaryHeap::with_capacity(letters_n);

    while let Some(Ok(line)) = input.next() {
        letters.push(Reverse(Node::Leaf(line.parse().unwrap())));
    }

    let root = huffman_algo(letters);

    println!("Max height: {}", root.max_height());
    println!("Min height: {}", root.min_height());
}

fn huffman_algo(mut frequencies: BinaryHeap<Reverse<Node>>) -> Node {
    if frequencies.len() == 2 {
        return Node::Intermediate(
            Box::new(frequencies.pop().unwrap().0),
            Box::new(frequencies.pop().unwrap().0),
        );
    }

    let new_node = Node::Intermediate(
        Box::new(frequencies.pop().unwrap().0),
        Box::new(frequencies.pop().unwrap().0),
    );

    frequencies.push(Reverse(new_node));

    huffman_algo(frequencies)
}
