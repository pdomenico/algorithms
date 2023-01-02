use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Job {
    weight: i32,
    length: i32,
}

fn read_from_file(filename: &str) -> Vec<Job> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file).lines();
    let mut ret = Vec::new();

    for (i, line) in reader.enumerate() {
        if i == 0 {
            continue;
        }
        if let Ok(l) = line {
            let values: Vec<&str> = l.split_whitespace().collect();
            ret.push(Job {
                weight: values[0].parse::<i32>().unwrap(),
                length: values[1].parse::<i32>().unwrap(),
            })
        }
    }
    ret
}

fn main() {
    let mut jobs = read_from_file("jobs.txt");

    jobs.sort_by(|a, b| {
        // weight - length score
        // let score_a = a.weight - a.length;
        // let score_b = b.weight - b.length;
        // match score_a.cmp(&score_b) {
        //     Ordering::Less => Ordering::Greater,
        //     Ordering::Greater => Ordering::Less,
        //     Ordering::Equal => match a.weight.cmp(&b.weight) {
        //         Ordering::Less => Ordering::Greater,
        //         _ => Ordering::Less,
        //     },
        // }

        // weight / length score
        let score_a = a.weight as f64 / a.length as f64;
        let score_b = b.weight as f64 / b.length as f64;
        if score_a > score_b {
            return Ordering::Less;
        }
        Ordering::Greater
    });

    let mut total_weighted_time: i64 = 0;
    let mut time_elapsed: i64 = 0;
    for job in jobs {
        time_elapsed += job.length as i64;
        total_weighted_time += job.weight as i64 * time_elapsed;
    }

    println!("{total_weighted_time}");
}
