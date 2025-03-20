use martos::task_manager::{task::TaskLoopFunctionType, TaskManager, TaskManagerTrait};

use martos::experiment::{
    add_task_experiment::add_task_experiment, delete_task_experiment::delete_task_experiment,
    get_task_count, set_task_count, wake_task_experiment::wake_task_experiment,
};
// what to do in no_std env
use martos::init_system;
use std::env::args;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn input_handler() -> String {
    let experiment_type = args().nth(1).expect("Can not find [experiment type] and [task count] arguments. Please restart program and put all arguments for experiment.");
    // let data_struct = args().nth(1).expect(
    //     "No arguments supplied. Please restart program and put all arguments for experiment.",
    // );

    let cnt = args().nth(2).expect("Can not find [task count] arguments. Please restart program and put all arguments for experiment.").parse::<usize>().unwrap();
    set_task_count(cnt);
    experiment_type
}
fn choose_experiment(experiment_type: String, // , task_count: usize
) -> TaskLoopFunctionType {
    let loop_fn = match experiment_type.as_str() {
        "add_task" => add_task_experiment,
        "delete_task" => delete_task_experiment,
        // "sleep_task" => sleep_task_experiment(task_count),
        "wake_task" => wake_task_experiment,
        // "combined" => combined_experiment(task_count),
        _ => panic!("Unknown experiment type: {}", experiment_type),
    };

    loop_fn
}
fn start_experiment(experiment_fn: TaskLoopFunctionType) {
    TaskManager::add_task(|| {}, experiment_fn, || true);
    TaskManager::experiment_start_task_manager();
}

fn main() {
    let
        // (
        experiment_type
    // ,
        // task_count)
        = input_handler();
    let experiment_fn = choose_experiment(
        experiment_type.clone(), // , // , task_count
    );
    init_system();
    let start_time = Instant::now();
    start_experiment(experiment_fn);
    let elapsed = start_time.elapsed();

    let mut result_file = File::create("experiment_results.txt").expect("Unable to create file.");
    writeln!(
        result_file,
        "Experiment type: {}, Task count: {}, Time Elapsed: {:?}",
        experiment_type,
        // data_struct,
        // QueueType
        get_task_count(),
        elapsed.as_secs()
    )
    .expect("Unable to write to file.");
    println!("Results written to experiment_results.txt");
}
