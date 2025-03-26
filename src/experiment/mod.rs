use lazy_static::lazy_static;

use crate::task_manager::NUM_PRIORITIES;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::sync::Mutex;

pub mod add_priority_task;
pub mod add_task;
pub mod delete_task;
pub mod put_to_sleep_task;
pub mod wake_up_task;

lazy_static! {
    static ref TASK_COUNT: Mutex<usize> = Mutex::new(0);
}

pub fn set_task_count(count: usize) {
    *TASK_COUNT.lock().unwrap() = count;
}

pub fn get_task_count() -> usize {
    *TASK_COUNT.lock().unwrap()
}

lazy_static! {
    static ref WORKED_TIME: Mutex<f64> = Mutex::new(0.0);
}

pub fn set_time(time: f64) {
    *WORKED_TIME.lock().unwrap() = time;
}

pub fn get_time() -> f64 {
    *WORKED_TIME.lock().unwrap()
}

/// Generates a vector of task priorities.
/// For the same `seed`, the sequence of priorities will be identical for independent program runs but appear in a chaotic order.
pub fn generate_priorities(seed: u64) -> Vec<usize> {
    // Initialize a pseudorandom number generator with a fixed seed (start value)
    // to ensure deterministic output for the same input.
    let mut generator = StdRng::seed_from_u64(seed);

    // Create a vector of priorities for all tasks.
    // Each task is assigned a random priority within the range [0, NUM_PRIORITIES).
    let priorities: Vec<usize> = (0..get_task_count())
        .map(|_| generator.random_range(0..NUM_PRIORITIES))
        .collect();
    priorities
}

/// Generates a vector of unique task ids.
/// For the same `seed`, the order of ids will be identical for independent program runs but shuffled.
pub fn generate_ids(seed: u64) -> Vec<usize> {
    // Initialize a pseudorandom number generator with a fixed seed
    // to ensure deterministic output for the same input.
    let mut generator = StdRng::seed_from_u64(seed);

    let task_count = get_task_count();

    // Create a vector of task ids: [2, 3, ..., task_count + 1].
    // The range starts from 2, because the task that init experiment has `id` 1.
    let mut ids: Vec<usize> = (2..=task_count + 1).collect();

    // Shuffle the vector to introduce randomness in the id order.
    ids.shuffle(&mut generator);
    ids
}
