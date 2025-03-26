use crate::experiment::{generate_priorities, set_time};
use crate::task_manager::TaskManager;
use std::time::Instant;

pub fn add_priority_task_experiment() {
    let priorities = generate_priorities(1);
    println!("Started adding tasks.");
    let start_time = Instant::now();

    for priority in priorities {
        TaskManager::add_priority_task(|| {}, || {}, || true, priority);
    }

    let elapsed = start_time.elapsed();
    println!("Finished adding tasks.\n----------------------------------\n");
    set_time(elapsed.as_secs_f64());
}
