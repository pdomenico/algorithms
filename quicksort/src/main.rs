use rand::{rngs::ThreadRng, Rng};
use std::time::{self, Instant};

fn quicksort(v: &mut [i32], length: usize, rng: &mut ThreadRng) {
    if length == 0 {
        return;
    }

    // choose the pivot at random
    let pivot = v[rng.gen_range(0..length)];
    let mut less_than_up_to: usize = 0;

    for i in 1..length {
        if v[i] < pivot {
            let tmp = v[i];
            v[i] = v[less_than_up_to + 1];
            v[less_than_up_to + 1] = tmp;
            less_than_up_to += 1;
        }
    }

    let tmp = v[less_than_up_to];
    v[less_than_up_to] = pivot;
    v[0] = tmp;

    quicksort(&mut v[0..less_than_up_to], less_than_up_to, rng);
    quicksort(
        &mut v[(less_than_up_to + 1)..length],
        length - (less_than_up_to + 1),
        rng,
    );
}

fn try_quicksort() {
    const ARRAY_SIZE: usize = 100000;
    let mut rng = rand::thread_rng();
    let mut array = [0; ARRAY_SIZE];
    for i in 0..ARRAY_SIZE {
        array[i] = rng.gen_range(0..(99999));
    }
    let now = Instant::now();
    quicksort(&mut array, ARRAY_SIZE, &mut rng);
    let elapsed_time = now.elapsed();
    println!("Took {}ms", elapsed_time.as_millis());
}

fn main() {
    for _ in 0..10 {
        try_quicksort();
    }
}
