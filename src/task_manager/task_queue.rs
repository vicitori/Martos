use crate::task_manager::cooperative::{CooperativeTask, TaskIdType};
use alloc::collections::{LinkedList, VecDeque};
use alloc::vec::Vec;

pub(crate) trait TaskQueue {
    fn push_task(&mut self, task: CooperativeTask);
    fn delete_task(&mut self, task_id: TaskIdType);
    fn get_task_by_id(&mut self, id: TaskIdType) -> Option<&mut CooperativeTask>;
    fn get_first_task_id(&mut self) -> Option<TaskIdType>;
    fn move_to_queue_end(&mut self, task_id: TaskIdType);
    fn make_clear(&mut self);
    fn does_empty(&self) -> bool;
}
#[cfg(feature = "vec_deque")]
impl TaskQueue for VecDeque<CooperativeTask> {
    fn push_task(&mut self, task: CooperativeTask) {
        self.push_back(task);
    }

    fn delete_task(&mut self, task_id: TaskIdType) {
        match self.iter().position(|task| task_id == task.id) {
            Some(index) => {
                self.remove(index);
            }
            None => {
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
            match self.remove(task_index) {
                Some(task) => self.push_back(task),
                None => panic!(
                    "Error: move_to_queue_end: Task with id {} index is out of bound.",
                    task_id
                ),
            }
        } else {
            panic!(
                "Error: move_to_queue_end: Can not find task with id {}.",
                task_id
            );
        }
    }

    fn make_clear(&mut self) {
        self.clear()
    }

    fn does_empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(feature = "vec")]
impl TaskQueue for Vec<CooperativeTask> {
    fn push_task(&mut self, task: CooperativeTask) {
        self.push(task);
    }

    fn delete_task(&mut self, task_id: TaskIdType) {
        if let Some(task_index) = self.iter().position(|task| task_id == task.id) {
            self.remove(task_index);
        } else {
            panic!(
                "Error: delete_task: Task with id {} not found in the task list.",
                task_id
            );
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

    fn make_clear(&mut self) {
        self.clear()
    }

    fn does_empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(feature = "linked_list")]
impl TaskQueue for LinkedList<CooperativeTask> {
    fn push_task(&mut self, task: CooperativeTask) {
        self.push_back(task);
    }

    fn delete_task(&mut self, task_id: TaskIdType) {
        self.retain(|task| task.id != task_id);
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

    fn move_to_queue_end(&mut self, _task_id: TaskIdType) {
        let mut split_list = self.split_off(0);
        while let Some(task) = split_list.pop_front() {
            self.push_back(task);
        }
    }

    fn make_clear(&mut self) {
        self.clear()
    }

    fn does_empty(&self) -> bool {
        self.is_empty()
    }
}
