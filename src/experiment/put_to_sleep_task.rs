use crate::experiment::{generate_ids, generate_priorities, set_time};
use crate::task_manager::TaskManager;
use std::time::Instant;

pub fn put_to_sleep_task_experiment() {
    let priorities = generate_priorities(5);
    println!("Started adding tasks.");

    for priority in priorities {
        TaskManager::add_priority_task(|| {}, || {}, || true, priority);
    }

    println!("Finished adding tasks.\n----------------------------------\n");
    // To have tasks wake up in a different sequence than the one they went to sleep in, different sets of ids
    let ids1: Vec<usize> = generate_ids(1);
    let ids2: Vec<usize> = generate_ids(10);
    println!("Started putting the tasks to sleep.");
    let start_time = Instant::now();

    for id in ids1 {
        TaskManager::put_to_sleep(id);
    }

    println!("Finished putting the tasks to sleep.\n----------------------------------\n");
    let elapsed = start_time.elapsed();
    set_time(elapsed.as_secs_f64());
    println!("Started waking up the tasks.");

    for id in ids2 {
        TaskManager::wake_up_task(id);
    }

    println!("Finishes waking up the tasks.\n----------------------------------\n");
}
