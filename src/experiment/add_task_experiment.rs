use crate::experiment::get_task_count;
use crate::task_manager::{TaskManager, NUM_PRIORITIES};

pub fn add_task_experiment() {
    for i in 0..get_task_count() {
        TaskManager::add_priority_task(|| {}, || {}, || true, i % NUM_PRIORITIES);
    }
}
