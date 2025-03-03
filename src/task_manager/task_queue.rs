use crate::task_manager::cooperative::{
    CooperativeTask, CooperativeTaskManager, TaskIdType, TaskPriorityType,
};
use crate::task_manager::task::{
    TaskLoopFunctionType, TaskSetupFunctionType, TaskStopConditionFunctionType,
};
use crate::task_manager::{TaskManager, TASK_MANAGER};

use alloc::collections::LinkedList;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

// идея -- параметризовать таск манагер структурой данных, а для
// каждой структуры данных добавить одно название для разных методов
// для этого 1. подумать, какие функции нужны
// 2. реализовать эти интерфейсы
// 3. добавить в таск менеджер параметризацию
pub trait TaskQueue {
    fn new() -> Self
    where
        Self: Sized;
    fn push_task(&mut self, task: CooperativeTask);
    fn delete_task(&mut self, task_id: TaskIdType);
    fn get_task(self, id: TaskIdType);
    fn get_len(self) -> usize;
}

impl TaskQueue for VecDeque<CooperativeTask> {
    fn new() -> Self
    where
        Self: Sized,
    {
        VecDeque::new()
    }

    fn push_task(&mut self, task: CooperativeTask) {
        self.push_back(task);
    }

    fn delete_task(&mut self, task_id: TaskIdType) {
        unsafe {
            if let Some(task_index) = self.iter().position(|task_vec| task_id == task_vec.id) {
                self.remove(task_index);
            } else {
                panic!(
                    "Error: delete_task: Task with id {} not found in the task list.",
                    task_id
                );
            }
        }
    }

    fn get_task(self, id: TaskIdType) {
        todo!()
    }

    fn get_len(self) -> usize {
        self.len()
    }
}

impl TaskQueue for Vec<CooperativeTask> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Vec::new()
    }

    fn push_task(&mut self, task: CooperativeTask) {
        self.push(task);
    }

    fn delete_task(&mut self, task_id: TaskIdType) {
        unsafe {
            if let Some(task_index) = self.iter().position(|task_vec| task_id == task_vec.id) {
                self.remove(task_index);
            } else {
                panic!(
                    "Error: delete_task: Task with id {} not found in the task list.",
                    task_id
                );
            }
        }
    }

    fn get_task(self, id: TaskIdType) {
        todo!()
    }

    fn get_len(self) -> usize {
        self.len()
    }
}

impl TaskQueue for LinkedList<CooperativeTask> {
    fn new() -> Self
    where
        Self: Sized,
    {
        LinkedList::new()
    }

    fn push_task(&mut self, task: CooperativeTask) {
        self.push_back(task);
    }

    // как удалить таски в связном списке?
    fn delete_task(&mut self, task_id: TaskIdType) {
        unsafe {
            if let Some(task_index) = self.iter().position(|task_vec| task_id == task_vec.id) {
                self.remove(task_index);
            } else {
                panic!(
                    "Error: delete_task: Task with id {} not found in the task list.",
                    task_id
                );
            }
        }
    }

    fn get_task(self, id: TaskIdType) {
        todo!()
    }

    fn get_len(self) -> usize {
        self.len()
    }
}
// impl TaskQueue for TaskManager {
//     fn push_to_queue(task: CooperativeTask) {
//         unsafe {
//             let priority = task.priority;
//
//             if TASK_MANAGER.tasks[priority].is_none() {
//                 TASK_MANAGER.tasks[priority] = Some(Vec::new());
//             }
//
//             TASK_MANAGER.tasks[priority].as_mut().unwrap().push(task);
//         }
//     }
//     fn move_to_queue_end(task: &mut CooperativeTask) {
//         let task_copy = task.clone();
//         CooperativeTaskManager::terminate_task(task.id);
//         CooperativeTaskManager::push_to_queue(task_copy);
//     }
//
//     fn delete_task(
//         task_id: crate::task_manager::cooperative::TaskIdType,
//         priority: crate::task_manager::cooperative::TaskPriorityType,
//     ) {
//         todo!()
//     }
// }
