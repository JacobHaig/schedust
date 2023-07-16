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
    email::EmailTask, parrallel::ParallelTask, print::PrintTask, script::ScriptTask,
    sequential::SequentialTask, Task,
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
        "1/10 * * * * *",
        "local",
        Task::Sequential(SequentialTask::new(vec![
            Task::Print(PrintTask::new("SENDING EMAIL")).to_ref(),
            Task::Email(EmailTask::new(
                vec!["abc@gmail.com", "admin@admin.com"],
                "ME",
                "Test Subject!",
                "Some Optional conditional message",
            ))
            .to_ref(),
            Task::Print(PrintTask::new("Completed Process1!")).to_ref(),
        ]))
        .to_ref(),
    )
    .to_ref();

    let process2 = Process::new(
        "process2",
        "1/3 * * * * *",
        "local",
        Task::Sequential(SequentialTask::new(vec![
            Task::Print(PrintTask::new("Starting Process2")).to_ref(),
            Task::Parallel(ParallelTask::new(vec![
                Task::Print(PrintTask::new("1")).to_ref(),
                Task::Print(PrintTask::new("2")).to_ref(),
                Task::Print(PrintTask::new("3")).to_ref(),
                Task::Print(PrintTask::new("4")).to_ref(),
                Task::Print(PrintTask::new("5")).to_ref(),
                Task::Print(PrintTask::new("6")).to_ref(),
                Task::Print(PrintTask::new("7")).to_ref(),
                Task::Print(PrintTask::new("8")).to_ref(),
                Task::Print(PrintTask::new("9")).to_ref(),
            ]))
            .to_ref(),
            Task::Print(PrintTask::new("Completed Process2!")).to_ref(),
        ]))
        .to_ref(),
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
    let p = scheduler.get_process_list().await;
    let s = serde_yaml::to_string(&p).unwrap();
    let mut f = File::create("results.yaml").unwrap();
    write!(f, "{}", s).unwrap();

    // scheduler.write_tasks_json("task.json").await;
    // scheduler.write_tasks_yaml("task.yaml").await;
}
