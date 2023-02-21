use std::{cmp::Ordering, fs::File, io, io::BufRead};

#[derive(Debug, Clone)]
struct City {
    id: usize,
    x: f64,
    y: f64,
}

impl City {
    fn distance(&self, other: &City) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let to_sqrt = x_diff * x_diff + y_diff * y_diff;
        f64::sqrt(to_sqrt)
    }
}

fn read_input(filename: &str) -> Vec<City> {
    let mut reader = io::BufReader::new(File::open(filename).unwrap()).lines();
    reader.next();
    let mut cities = Vec::new();

    while let Some(Ok(line)) = reader.next() {
        cities.push(City {
            id: line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .unwrap()
                - 1,
            x: line.split_whitespace().nth(1).unwrap().parse().unwrap(),
            y: line.split_whitespace().nth(2).unwrap().parse().unwrap(),
        });
    }

    cities
}

fn main() {
    let cities = read_input("input.txt");

    let first = &cities[0];
    let mut from = first;
    let mut n_visited = 1;
    let mut total_distance = 0f64;
    let mut visited = vec![false; cities.len()];
    visited[from.id] = true;

    while n_visited < cities.len() {
        println!("visited: {}", n_visited);
        let next = cities
            .iter()
            .filter_map(|city| {
                if visited[city.id] {
                    None
                } else {
                    Some((city, city.distance(from)))
                }
            })
            .min_by(|(city1, distance1), (city2, distance2)| {
                let comp = distance1.total_cmp(distance2);
                return if let Ordering::Equal = comp {
                    city1.id.cmp(&city2.id)
                } else {
                    comp
                };
            })
            .unwrap();

        total_distance = total_distance + next.1;
        n_visited += 1;
        visited[next.0.id] = true;
        from = next.0;
    }

    total_distance = total_distance + from.distance(first);
    println!("Total distance: {}", total_distance);
}
