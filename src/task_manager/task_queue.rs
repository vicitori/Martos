use crate::task_manager::cooperative::{CooperativeTask, TaskIdType};

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
    fn get_task_by_id(&mut self, id: TaskIdType) -> Option<&mut CooperativeTask>;
    fn get_first_task_id(&mut self) -> Option<TaskIdType>;
    fn move_to_queue_end(&mut self, task_id: TaskIdType);
}

impl TaskQueue for VecDeque<CooperativeTask> {
    fn new() -> Self {
        VecDeque::new()
    }

    fn push_task(&mut self, task: CooperativeTask) {
        self.push_back(task);
    }

    fn delete_task(&mut self, task_id: TaskIdType) {
        unsafe {
            if let Some(task_index) = self.iter().position(|task| task_id == task.id) {
                self.remove(task_index);
            } else {
                panic!(
                    "Error: delete_task: Task with id {} not found in the task list.",
                    task_id
                );
            }
        }
    }
    fn get_task_by_id(&mut self, id: TaskIdType) -> Option<&mut CooperativeTask> {
        self.iter_mut().find(|task| task.id == id)
    }

    fn get_first_task_id(&mut self) -> Option<TaskIdType> {
        if let Some(task) = self.front() {
            return Some(task.id);
        }
        None
    }

    fn move_to_queue_end(&mut self, task_id: TaskIdType) {
        if let Some(task_index) = self.iter().position(|task| task.id == task_id) {
            let task = self.remove(task_index);
            self.push(task);
        } else {
            panic!(
                "Error: move_to_queue_end: Can not find task with id {}.",
                task_id
            );
        }
    }
}
impl TaskQueue for Vec<CooperativeTask> {
    fn new() -> Self {
        Vec::new()
    }

    fn push_task(&mut self, task: CooperativeTask) {
        self.push(task);
    }

    fn delete_task(&mut self, task_id: TaskIdType) {
        unsafe {
            if let Some(task_index) = self.iter().position(|task| task_id == task.id) {
                self.remove(task_index);
            } else {
                panic!(
                    "Error: delete_task: Task with id {} not found in the task list.",
                    task_id
                );
            }
        }
    }
    fn get_task_by_id(&mut self, id: TaskIdType) -> Option<&mut CooperativeTask> {
        self.iter_mut().find(|task| task.id == id)
    }

    fn get_first_task_id(&mut self) -> Option<TaskIdType> {
        if let Some(task) = self.first() {
            return Some(task.id);
        }
        None
    }

    fn move_to_queue_end(&mut self, task_id: TaskIdType) {
        if let Some(task_index) = self.iter().position(|task| task.id == task_id) {
            let task = self.remove(task_index);
            self.push(task);
        } else {
            panic!(
                "Error: move_to_queue_end: Can not find task with id {}.",
                task_id
            );
        }
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

    fn delete_task(&mut self, task_id: TaskIdType) {
        unsafe {
            let mut split_list = self.split_off(0);
            while let Some(task) = split_list.pop_front() {
                if task.id != task_id {
                    self.push_back(task);
                }
            }
        }
    }

    fn get_task_by_id(&mut self, id: TaskIdType) -> Option<&mut CooperativeTask> {
        self.iter_mut().find(|task| task.id == id)
    }

    fn get_first_task_id(&mut self) -> Option<TaskIdType> {
        if let Some(task) = self.front() {
            return Some(task.id);
        }
        None
    }

    fn move_to_queue_end(&mut self, task_id: TaskIdType) {
        let mut split_list = self.split_off(0);
        while let Some(task) = split_list.pop_front() {
            self.push_back(task);
        }
    }
}
