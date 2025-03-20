extern crate alloc;

use crate::task_manager::{
    cooperative::QueueType,
    task::{TaskLoopFunctionType, TaskSetupFunctionType, TaskStopConditionFunctionType},
};

/// Number of existing priorities.
pub const NUM_PRIORITIES: usize = 11;
pub mod task;
pub(crate) mod task_queue;

cfg_if::cfg_if! {
    if #[cfg(feature = "prmptive")] {
        pub(crate) mod preemptive;
        pub type TaskManager = preemptive::PreemptiveTaskManager;
    } else {
        mod cooperative;
        pub type TaskManager = cooperative::CooperativeTaskManager<QueueType>;
    }
}

/// Operating system task manager.
/// By default, [cooperative::CooperativeTaskManager] is used
pub static mut TASK_MANAGER: TaskManager = TaskManager::new();

pub trait TaskManagerTrait {
    /// Add task to task manager. You should pass setup, loop and condition functions.
    fn add_task(
        setup_fn: TaskSetupFunctionType,
        loop_fn: TaskLoopFunctionType,
        stop_condition_fn: TaskStopConditionFunctionType,
    );

    /// Starts task manager work.
    fn start_task_manager() -> !;
}
