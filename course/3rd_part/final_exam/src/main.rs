use rand::Rng;

fn main() {
    let (mut product, mut proc_time, mut deadline) = (0, 0, 0);
    for _ in 0..100 {
        let (lateness_prod, lateness_deadline, lateness_proc_time) = question6();

        if lateness_prod < lateness_proc_time && lateness_prod < lateness_deadline {
            product += 1;
        } else if lateness_proc_time < lateness_prod && lateness_proc_time < lateness_deadline {
            proc_time += 1;
        } else {
            deadline += 1;
        }
    }
    println!(
        "Product: {}, Proc time: {}, Deadline: {}",
        product, proc_time, deadline
    );
}

#[derive(Clone)]
struct Job {
    proc_time: u32,
    deadline: u32,
}

fn question6() -> (u32, u32, u32) {
    let jobs_n = 100;
    let mut rng = rand::thread_rng();
    let mut jobs = Vec::new();

    for _ in 0..jobs_n {
        jobs.push(Job {
            proc_time: rng.gen_range(1..100),
            deadline: rng.gen_range(20..400),
        });
    }

    let mut by_product = jobs.clone();
    by_product.sort_by_key(|job| job.proc_time * job.deadline);

    let mut by_deadline = jobs.clone();
    by_deadline.sort_by_key(|job| job.deadline);

    let mut by_proc_time = jobs.clone();
    by_proc_time.sort_by_key(|job| job.proc_time);

    let mut completion_time = 0;
    let (mut lateness_prod, mut lateness_deadline, mut lateness_proc_time) = (0, 0, 0);

    by_product.iter().for_each(|job| {
        completion_time += job.proc_time;
        if completion_time > job.deadline {
            lateness_prod += completion_time - job.deadline;
        }
    });

    completion_time = 0;

    by_deadline.iter().for_each(|job| {
        completion_time += job.proc_time;
        if completion_time > job.deadline {
            lateness_deadline += completion_time - job.deadline;
        }
    });

    completion_time = 0;

    by_proc_time.iter().for_each(|job| {
        completion_time += job.proc_time;
        if completion_time > job.deadline {
            lateness_proc_time += completion_time - job.deadline;
        }
    });

    (lateness_prod, lateness_deadline, lateness_proc_time)
}
