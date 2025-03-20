use crate::experiment::get_task_count;
use crate::task_manager::TaskManager;

pub fn wake_task_experiment() {
    let task_ids: Vec<usize> = (0..get_task_count()).map(|i| i).collect();
    for id in &task_ids {
        TaskManager::put_to_sleep(*id);
    }
    for id in task_ids {
        TaskManager::wake_up_task(id);
    }
}
