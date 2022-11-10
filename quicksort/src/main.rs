fn quicksort(v: &mut [i32], length: usize) {
    if length == 0 {
        return;
    }

    // choose the pivot to be the first element
    let pivot = v[0];
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

    quicksort(&mut v[0..less_than_up_to], less_than_up_to);
    quicksort(
        &mut v[(less_than_up_to + 1)..length],
        length - (less_than_up_to + 1),
    );
}

fn main() {
    let mut v = [
        32, 324, 235, 43, 32, 4, 32, 43, 3, 423, 4, 3, 423, 4, 32, 4, 32, 6, 5, 7, 8, 8767, 78,
    ];

    quicksort(&mut v, 23);
    for i in 0..23 {
        println!("{}", v[i]);
    }
}
