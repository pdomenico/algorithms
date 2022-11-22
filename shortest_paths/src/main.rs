use std::collections::HashMap;

fn shortest_paths<'a>(
    start: &'a str,
    graph: HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, i32> {
    let mut paths: HashMap<&str, i32> = HashMap::new();
    let mut queue = vec![start];

    while queue.len() > 0 {
        let elem = queue.pop().unwrap();
        if elem == start {
            paths.insert(elem.clone(), 0);
        }

        if let Some(subnodes) = graph.get(elem) {
            for subnode in subnodes {
                if !paths.contains_key(subnode) {
                    queue.push(subnode);
                    paths.insert(subnode, paths.get(elem).unwrap() + 1);
                }
            }
        }
    }
    paths
}

fn main() {
    let graph: HashMap<&str, Vec<&str>> = HashMap::from([
        ("s", vec!["a", "b"]),
        ("a", vec!["s", "c"]),
        ("b", vec!["s", "c", "d"]),
        ("c", vec!["a", "b", "d", "e"]),
        ("d", vec!["b", "c", "e"]),
        ("e", vec!["c", "d"]),
    ]);

    println!("{:?}", shortest_paths("s", graph));
}
