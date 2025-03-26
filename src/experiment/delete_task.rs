use crate::experiment::{generate_ids, generate_priorities, set_time};
use crate::task_manager::TaskManager;
use std::time::Instant;

pub fn delete_task_experiment() {
    let priorities = generate_priorities(1);
    println!("Started adding tasks.");

    for priority in priorities {
        TaskManager::add_priority_task(|| {}, || {}, || true, priority);
    }

    println!("Finished adding tasks.\n----------------------------------\n");
    let ids: Vec<usize> = generate_ids(1);
    println!("Started deleting tasks.");
    let start_time = Instant::now();

    for id in ids {
        TaskManager::delete_task(id);
    }

    let elapsed = start_time.elapsed();
    println!("Finished deleting tasks.\n----------------------------------\n");
    set_time(elapsed.as_secs_f64());
}
