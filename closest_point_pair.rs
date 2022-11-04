use std::cmp::min;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

fn sort_by_coord(points: &mut Vec<Point>, coord: &str) -> Vec<Point> {
    let len = points.len();
    if len == 1 {
        return points.clone();
    }

    let (left, right) = points.split_at_mut(len / 2);
    let left = sort_by_coord(&mut left.to_vec(), coord);
    let right = sort_by_coord(&mut right.to_vec(), coord);
    return merge(&left, &right, coord);
}

fn merge(left: &Vec<Point>, right: &Vec<Point>, coord: &str) -> Vec<Point> {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut res: Vec<Point> = Vec::new();

    while (left_index < left.len()) && (right_index < right.len()) {
        if coord == "x" {
            if left[left_index].x > right[right_index].x {
                res.push(right[right_index]);
                right_index += 1;
            } else {
                res.push(left[left_index]);
                left_index += 1;
            }
        } else {
            if left[left_index].y > right[right_index].y {
                res.push(right[right_index]);
                right_index += 1;
            } else {
                res.push(left[left_index]);
                left_index += 1;
            }
        }
    }

    if left_index < left.len() {
        res.extend_from_slice(&left[left_index..]);
    } else if right_index < right.len() {
        res.extend_from_slice(&right[right_index..]);
    }

    return res;
}

fn find_closest_pair(points: &Vec<Point>) -> Option<(Point, Point)> {
    if points.len() == 2 {
        return Some((points[0], points[1]));
    }

    if points.len() == 1 {
        return None;
    }

    let (left, right) = points.split_at(points.len() / 2);
    let closest_pair_left = find_closest_pair(&left.to_vec());
    let closest_pair_right = find_closest_pair(&right.to_vec());
    let mut delta: f64;
    match (closest_pair_left, closest_pair_right) {
        (Some((p1, p2)), Some((p3, p4))) => {
            let d1 = distance(&p1, &p2);
            let d2 = distance(&p3, &p4);
            delta = d1.min(d2);
        }
        (Some((p1, p2)), None) => {
            delta = distance(&p1, &p2);
        }
        (None, Some((p3, p4))) => {
            delta = distance(&p3, &p4);
        }
        (None, None) => {
            delta = distance(&points[0], &points[1]);
        }
    }

    let x_middle = points[points.len() / 2].x;
    let mut strip: Vec<Point> = Vec::new();
    for point in points {
        if (point.x - x_middle).abs() < delta {
            strip.push(*point);
        }
    }
    let strip = sort_by_coord(&mut strip, "y");
    for i in 0..strip.len() {
        for j in (i + 1)..min(i + 7, strip.len()) {
            let d = distance(&strip[i], &strip[j]);
            if d < delta {
                return Some((strip[i], strip[j]));
            }
        }
    }
    
    // return the closest pair
    match (closest_pair_left, closest_pair_right) {
        (Some((p1, p2)), Some((p3, p4))) => {
            let d1 = distance(&p1, &p2);
            let d2 = distance(&p3, &p4);
            if d1 < d2 {
                return Some((p1, p2));
            } else {
                return Some((p3, p4));
            }
        }
        (Some((p1, p2)), None) => {
            return Some((p1, p2));
        }
        (None, Some((p3, p4))) => {
            return Some((p3, p4));
        }
        (None, None) => {
            return Some((points[0], points[1]));
        }
    }
    
}

// try the sorting algo with a sample vector of points
fn main() {
    let mut points = vec![
        Point { x: 165.0, y: 45.0 },
        Point { x: 2.06, y: 435.0 },
        Point { x: 2.0, y: 65.0 },
        Point { x: 46.0, y: 123.0 },
        Point { x: 576.0, y: 65.0 },
        Point { x: 161.0, y: 1236.0 },
        Point { x: 777.0, y: 763.0 },
        Point { x: 82.0, y: 185.0 },
        Point { x: 91.0, y: 911.0 },
        Point { x: 1000.0, y: 100.0 },
    ];

    let sorted_points = sort_by_coord(&mut points, "x");
    // find the closest pair in the sorted vector
    let closest_pair = find_closest_pair(&sorted_points);
    println!("Closest pair: {:?}", closest_pair);
}