// Scheduler, and trait for .seconds(), .minutes(), etc.
// use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
// use clokwerk::{Interval, Scheduler};
// use job_scheduler::{Job, JobScheduler};
// use std::fs::write;
// use serde::{Deserialize, Serialize};
// use std::process::Command;

// use std::sync::Arc;
// use std::sync::Mutex;

mod process;
mod scheduler;
mod server;
mod tasks;

use std::{fs::File, time::Duration};

use scheduler::Processes;

use process::Process;

use crate::tasks::{
    delay::DelayTask, email::EmailTask, parrallel::ParallelTask, print::PrintTask,
    script::ScriptTask, sequential::SequentialTask, Task,
};

// use task::Task;
// use once_cell::sync::Lazy;

// static mut RUNNING: Lazy<bool> = Lazy::new(|| true);

#[tokio::main]
async fn main() {
    // let running = Arc::new(AtomicBool::new(true));
    // let running = Arc::new(AtomicBool::new(true));
    // let r = running.clone();

    // ctrlc::set_handler(|| unsafe {
    //     *RUNNING = false;
    // })
    // .expect("Error setting Ctrl-C handler");

    /*
    println!("Waiting for Ctrl-C...");
    while unsafe { *RUNNING } {
        // Keep running until Ctrl-C is pressed
    }
    println!("Exiting...");
    */

    // let a = tasks::print::Print::new("Hello");

    // Create a new task
    let process1 = Process::new(
        "process1",
        "* * * * * *",
        "local",
        SequentialTask::new(vec![
            PrintTask::new("SENDING EMAIL").to_task(),
            EmailTask::new(
                vec!["abc@gmail.com", "admin@admin.com"],
                "ME",
                "Test Subject!",
                "Some Optional conditional message",
            )
            .to_task(),
            PrintTask::new("Completed Process1!").to_task(),
        ])
        .to_task(),
    )
    .to_ref();

    let process2 = Process::new(
        "process2",
        "1/3 * * * * *",
        "local",
        SequentialTask::new(vec![
            PrintTask::new("Starting Process2").to_task(),
            SequentialTask::new(vec![
                PrintTask::new("1").to_task(),
                DelayTask::new(Duration::from_secs_f32(0.5)).to_task(),
                PrintTask::new("2").to_task(),
                DelayTask::new(Duration::from_secs_f32(0.5)).to_task(),
                PrintTask::new("3").to_task(),
                DelayTask::new(Duration::from_secs_f32(0.5)).to_task(),
                PrintTask::new("4").to_task(),
                DelayTask::new(Duration::from_secs_f32(0.5)).to_task(),
                PrintTask::new("5").to_task(),
                DelayTask::new(Duration::from_secs_f32(0.5)).to_task(),
                PrintTask::new("6").to_task(),
                DelayTask::new(Duration::from_secs_f32(0.5)).to_task(),
                PrintTask::new("7").to_task(),
                DelayTask::new(Duration::from_secs_f32(0.5)).to_task(),
                PrintTask::new("8").to_task(),
                DelayTask::new(Duration::from_secs_f32(0.5)).to_task(),
                PrintTask::new("9").to_task(),
            ])
            .to_task(),
            PrintTask::new("Completed Process2!").to_task(),
        ])
        .to_task(),
    )
    .to_ref();

    // Create a new scheduler
    let mut scheduler = scheduler::Scheduler::new().await;

    // Add the task to the scheduler
    scheduler.add_process(process1.clone()).await;
    scheduler.add_process(process2.clone()).await;

    // Run the web-server
    let process_list: Processes = scheduler.get_process_list().await;
    tokio::task::spawn(server::start(process_list.clone()));

    for _ in 0..40 {
        scheduler.run_once().await;
        tokio::time::sleep(Duration::from_secs_f32(0.5)).await;
    }

    use std::io::Write;
    let processes = scheduler.get_process_list().await;
    let s = serde_yaml::to_string(&processes).unwrap();
    let mut file = File::create("results.yaml").unwrap();
    write!(file, "{}", s).unwrap();

    // scheduler.write_tasks_json("task.json").await;
    // scheduler.write_tasks_yaml("task.yaml").await;
}
