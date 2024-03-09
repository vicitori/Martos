use core::array::from_fn;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// The number of tasks can fit into a type usize.
pub type TaskNumberType = usize;

// TODO: rewrite with cfg!
#[cfg(not(feature = "c-library"))]
/// Type of setup function, that is called once at the beginning of task.
type TaskSetupFunctionType = fn() -> ();
#[cfg(feature = "c-library")]
/// Type of setup function, that is called once at the beginning of task.
type TaskSetupFunctionType = extern "C" fn() -> ();
#[cfg(not(feature = "c-library"))]
/// Type of loop function, that is called in loop.
type TaskLoopFunctionType = fn() -> ();
#[cfg(feature = "c-library")]
/// Type of loop function, that is called in loop.
type TaskLoopFunctionType = extern "C" fn() -> ();
#[cfg(not(feature = "c-library"))]
/// Type of condition function for stopping loop function execution.
type TaskStopConditionFunctionType = fn() -> bool;
#[cfg(feature = "c-library")]
/// Type of condition function for stopping loop function execution.
type TaskStopConditionFunctionType = extern "C" fn() -> bool;

/// Max number of tasks.
// TODO: Should be dynamic array of tasks.
const MAX_NUMBER_OF_TASKS: TaskNumberType = 20;

#[repr(C)]
/// Task representation for task manager.
struct Task {
    /// Setup function, that is called once at the beginning of task.
    setup_fn: TaskSetupFunctionType,
    /// Loop function, that is called in loop.
    loop_fn: TaskLoopFunctionType,
    /// Condition function for stopping loop function execution.
    stop_condition_fn: TaskStopConditionFunctionType,
}

#[repr(C)]
/// Future shell for task for execution.
struct FutureTask {
    /// Task to execute in task manager.
    task: Task,
    /// Marker for setup function completion.
    is_setup_completed: bool,
}

impl Future for FutureTask {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut array: [usize; 8] = core::array::from_fn(|i| i);
        array[0] = 5;
        if (self.task.stop_condition_fn)() {
            Poll::Ready(())
        } else {
            if !self.is_setup_completed {
                (self.task.setup_fn)();
                self.is_setup_completed = true;
            } else {
                (self.task.loop_fn)();
            }
            Poll::Pending
        }
    }
}

/// Creates simple task waker. May be more difficult in perspective.
fn task_waker() -> Waker {
    fn raw_clone(_: *const ()) -> RawWaker {
        RawWaker::new(0 as *const (), &NOOP_WAKER_VTABLE)
    }

    fn raw_wake(_: *const ()) {}

    fn raw_wake_by_ref(_: *const ()) {}

    fn raw_drop(_: *const ()) {}

    static NOOP_WAKER_VTABLE: RawWakerVTable =
        RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

    let raw_waker = RawWaker::new(0 as *const (), &NOOP_WAKER_VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

#[repr(C)]
/// Task executor representation. Based on round-robin scheduling without priorities.
pub struct TaskExecutor {
    /// Static array of tasks to execute.
    tasks: [FutureTask; MAX_NUMBER_OF_TASKS],
    /// Index of task, that should be executed.
    task_to_execute_index: TaskNumberType,
    /// Number of tasks to execute.
    tasks_number: TaskNumberType,
}

impl TaskExecutor {
    /// Creates new task executor.
    pub fn new() -> TaskExecutor {
        #[cfg(not(feature = "c-library"))]
        fn setup_fn() {}
        #[cfg(not(feature = "c-library"))]
        fn loop_fn() {}
        #[cfg(not(feature = "c-library"))]
        fn stop_condition_fn() -> bool {
            return true;
        }
        #[cfg(feature = "c-library")]
        extern "C" fn setup_fn() {}
        #[cfg(feature = "c-library")]
        extern "C" fn loop_fn() {}
        #[cfg(feature = "c-library")]
        extern "C" fn stop_condition_fn() -> bool {
            return true;
        }

        let tasks: [FutureTask; MAX_NUMBER_OF_TASKS] = from_fn(|_| {
            let task = Task {
                setup_fn: setup_fn,
                loop_fn: loop_fn,
                stop_condition_fn: stop_condition_fn,
            };
            FutureTask {
                task: task,
                is_setup_completed: true,
            }
        });

        TaskExecutor {
            tasks: tasks,
            task_to_execute_index: 0,
            tasks_number: 0,
        }
    }

    /// Add task to task executor. You should pass setup, loop and condition functions.
    // TODO: Maybe it is better to pass tasks here. Functions are done for C compatibility.
    pub fn add_task(
        &mut self,
        setup_fn: TaskSetupFunctionType,
        loop_fn: TaskLoopFunctionType,
        stop_condition_fn: TaskStopConditionFunctionType,
    ) {
        let task = Task {
            setup_fn,
            loop_fn,
            stop_condition_fn,
        };
        let future_task = FutureTask {
            task,
            is_setup_completed: false,
        };
        self.tasks[self.tasks_number] = future_task;
        self.tasks_number += 1
    }

    /// Starts task manager work.
    // TODO: Support priorities.
    // TODO: Delete tasks from task vector if they are pending
    pub fn start_task_manager(&mut self) -> ! {
        loop {
            if !self.tasks.is_empty() {
                let waker = task_waker();
                let task = &mut self.tasks[self.task_to_execute_index];
                let mut task_future_pin = Pin::new(task);
                let _ = task_future_pin
                    .as_mut()
                    .poll(&mut Context::from_waker(&waker));

                if self.task_to_execute_index + 1 < self.tasks.len() {
                    self.task_to_execute_index += 1;
                } else {
                    self.task_to_execute_index = 0;
                }
            }
        }
    }
}