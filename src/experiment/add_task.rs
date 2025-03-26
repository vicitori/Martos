use crate::experiment::{get_task_count, set_time};
use crate::task_manager::{TaskManager, TaskManagerTrait};
use std::time::Instant;

pub fn add_task_experiment() {
    let task_count = get_task_count();
    println!("Started adding tasks.");
    let start_time = Instant::now();

    for _ in 0..task_count {
        TaskManager::add_task(|| {}, || {}, || true);
    }

    let elapsed = start_time.elapsed();
    println!("Finished adding tasks.\n----------------------------------\n");
    set_time(elapsed.as_secs_f64());
}
