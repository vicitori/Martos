use alloc::collections::LinkedList;
pub struct ListCooperativeTaskManager {
    base: BaseCooperativeTaskManager,
    tasks: [Option<LinkedList<Task>>; NUM_PRIORITIES],
}

impl CooperativeTaskManager for ListCooperativeTaskManager {
    fn delete_task(task_id: TaskIdType, priority: TaskPriorityType) {
        unsafe {
            let Some(vec) = TASK_MANAGER.tasks[priority].as_mut() else {
                panic!(
                    "Error:delete_task: Task with id {} does not exist in priority {}.",
                    task_id, priority
                );
            };
            if let Some(task_index) = vec.iter().position(|task_vec| task_id == task_vec.id) {
                vec.remove(task_index);
            } else {
                panic!(
                    "Error: delete_task: Task with id {} not found in the task list.",
                    task_id
                );
            }
        }
    }

    fn get_id_by_position(priority: TaskPriorityType, position: usize) -> TaskIdType {
        if priority >= NUM_PRIORITIES {
            panic!("Error: get_id_by_priorities: Task's priority {} is invalid. It must be between 0 and {}.", priority, NUM_PRIORITIES);
        }
        unsafe {
            if TASK_MANAGER.tasks[priority].is_none() {
                panic!(
                    "Error: get_id_by_position: No tasks found with priority {}.",
                    priority
                );
            }
            if TASK_MANAGER.tasks[priority].as_ref().unwrap().len() - 1 < position {
                panic!(
                    "Error: get_id_by_position: No tasks found for task on position {}.",
                    position
                );
            }
            TASK_MANAGER.tasks[priority]
                .as_ref()
                .unwrap()
                .get(position)
                .unwrap()
                .id
        }
    }

    fn push_to_queue(task: CooperativeTask) {
        unsafe {
            let priority = task.priority;

            if TASK_MANAGER.tasks[priority].is_none() {
                TASK_MANAGER.tasks[priority] = Some(Vec::new());
            }

            TASK_MANAGER.tasks[priority].as_mut().unwrap().push(task);
        }
    }

    fn reset_task_manager() {
        unsafe {
            for priority in 0..NUM_PRIORITIES {
                if let Some(vec) = TASK_MANAGER.tasks[priority].as_mut() {
                    vec.clear();
                    TASK_MANAGER.tasks[priority] = None;
                }
            }
            TASK_MANAGER.next_task_id = 0;
            TASK_MANAGER.exec_task_id = None;
        }
    }

    fn is_empty() -> bool {
        unsafe {
            for vec_opt in TASK_MANAGER.tasks.iter() {
                if vec_opt.is_some() {
                    return false;
                }
            }
            true
        }
    }

    /// Count tasks of the specified priority.
    fn count_tasks_with_priority(priority: TaskPriorityType) -> usize {
        if priority >= NUM_PRIORITIES {
            panic!("Error: count_tasks_with_priority: Task's priority {} is invalid. It must be between 0 and {}.", priority, NUM_PRIORITIES);
        }
        unsafe {
            if let Some(vec) = TASK_MANAGER.tasks[priority].as_ref() {
                vec.len()
            } else {
                0
            }
        }
    }

    /// Count all tasks in task manager.
    fn count_all_tasks() -> usize {
        unsafe {
            TASK_MANAGER
                .tasks
                .iter()
                .flatten() // Skip None
                .map(|vec| vec.len())
                .sum()
        }
    }
}
