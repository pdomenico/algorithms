use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::BufRead;
use std::time::Instant;
use std::{fs::File, io};

const MAX_CITY_N: usize = 30;

#[derive(Clone)]
struct City {
    id: usize,
    x: f64,
    y: f64,
    distances: Vec<Option<f64>>,
}

impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for City {}

impl PartialOrd for City {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for City {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for City {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl City {
    fn distance_from(&mut self, other_city: &City) -> f64 {
        if let Some(dist) = self.distances[other_city.id] {
            return dist;
        }
        let a = f64::powi(self.x - other_city.x, 2);
        let b = f64::powi(self.y - other_city.y, 2);
        let distance = f64::sqrt(a + b);
        self.distances[other_city.id] = Some(distance);
        distance
    }
}

fn read_input(filename: &str) -> Vec<City> {
    let file = File::open(filename).unwrap();
    let mut reader = io::BufReader::new(file).lines();
    reader.next();
    let mut cities = Vec::new();
    let mut id = 0;
    while let Some(Ok(line)) = reader.next() {
        let mut coords = line.split_whitespace();
        cities.push(City {
            id,
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
            distances: vec![None; MAX_CITY_N],
        });
        id += 1;
    }

    cities
}

fn city_set_to_key(cities: &Vec<usize>) -> usize {
    let mut n = 0usize;
    for city in cities {
        n = n | (1 << city - 1);
    }
    n
}

fn tsp(cities: &mut Vec<City>, start: usize) -> f64 {
    let mut cities_clone = cities.clone();
    let mut cities_without_start = [0usize; MAX_CITY_N];
    let mut i = 0;
    for (j, city) in cities.iter().enumerate() {
        if j != start {
            cities_without_start[i] = city.id;
            i += 1;
        }
    }

    let mut solutions: Vec<[f64; MAX_CITY_N]> =
        vec![[f64::INFINITY; MAX_CITY_N]; usize::pow(2, cities.len() as u32 - 1)];

    for set_size in 1..cities.len() {
        println!("Set size {}", set_size);
        for set in cities_without_start[0..(cities.len() - 1)]
            .iter()
            .map(|x| *x)
            .combinations(set_size)
        {
            for dest in set.iter() {
                let key = city_set_to_key(&set);
                if set.len() == 1 {
                    let distance = cities[start].distance_from(&cities_clone[*dest]);
                    solutions[key][*dest] = distance;
                    continue;
                }

                let set_without_dest = set
                    .iter()
                    .filter(|x| *x != dest)
                    .map(|x| *x)
                    .collect::<Vec<_>>();

                let shortest = set_without_dest
                    .iter()
                    .map(|intermediate_city| {
                        solutions[city_set_to_key(&set_without_dest)][*intermediate_city]
                            + cities[*intermediate_city].distance_from(&cities_clone[*dest])
                    })
                    .fold(f64::INFINITY, f64::min);

                solutions[key][*dest] = shortest;
            }
        }
    }

    let final_solutions =
        solutions[city_set_to_key(&cities_without_start[0..(cities.len() - 1)].to_vec())];
    let mut shortest = f64::INFINITY;
    for dest in cities_without_start[0..(cities.len() - 1)].iter() {
        let length = final_solutions[*dest] + cities[start].distance_from(&cities_clone[*dest]);
        if length < shortest {
            shortest = length;
        }
    }
    shortest
}

fn main() {
    let mut cities = read_input("test1.txt");

    println!("Final solution: {}", tsp(&mut cities, 0));
}
