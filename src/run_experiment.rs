use martos::experiment::put_to_sleep_task::put_to_sleep_task_experiment;
use martos::experiment::{
    add_priority_task::add_priority_task_experiment, add_task::add_task_experiment,
    delete_task::delete_task_experiment, get_task_count, get_time, set_task_count,
    wake_up_task::wake_up_task_experiment,
};
use martos::task_manager::{task::TaskLoopFunctionType, TaskManager, TaskManagerTrait};
use std::{env::args, fs::OpenOptions, io::Write};

fn input_handler() -> String {
    let experiment_type = args().nth(1).expect("Can not find [experiment type] and [task count] arguments. Please restart program and put all arguments for experiment.");
    let cnt = args().nth(2).expect("Can not find [task count] arguments. Please restart program and put all arguments for experiment.").parse::<usize>().unwrap();
    set_task_count(cnt);
    experiment_type
}

fn choose_experiment(experiment_type: String) -> TaskLoopFunctionType {
    let loop_fn = match experiment_type.as_str() {
        "add_task" => add_task_experiment,
        "add_prio_task" => add_priority_task_experiment,
        "delete_task" => delete_task_experiment,
        "sleep_task" => put_to_sleep_task_experiment,
        "wake_task" => wake_up_task_experiment,
        _ => panic!("Unknown experiment type: {}.", experiment_type),
    };
    loop_fn
}
fn start_experiment(experiment_fn: TaskLoopFunctionType) {
    TaskManager::add_task(|| {}, experiment_fn, || true);
    TaskManager::experiment_start_task_manager();
}

fn main() {
    let experiment_type = input_handler();
    let experiment_fn = choose_experiment(experiment_type.clone());
    start_experiment(experiment_fn);

    let mut result_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("experiment_results.txt")
        .expect("Unable to open file.");
    let data_type = if cfg!(feature = "vec") {
        "Vec"
    } else if cfg!(feature = "vec_deque") {
        "VecDeque"
    } else if cfg!(feature = "linked_list") {
        "LinkedList"
    } else {
        panic!("No data type feature enabled!");
    };
    writeln!(
        result_file,
        "Experiment type: {}, Data struct: {}, Task count: {}, Time Elapsed: {:?}",
        experiment_type,
        data_type,
        get_task_count(),
        get_time()
    )
    .expect("Unable to write to file.");
    println!("Results written to experiment_results.txt.");
}
