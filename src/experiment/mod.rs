use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod add_task_experiment;
pub mod delete_task_experiment;
pub mod wake_task_experiment;

lazy_static! {
    static ref TASK_COUNT: Mutex<usize> = Mutex::new(0);
}

pub fn set_task_count(count: usize) {
    *TASK_COUNT.lock().unwrap() = count;
}

pub fn get_task_count() -> usize {
    *TASK_COUNT.lock().unwrap()
}
