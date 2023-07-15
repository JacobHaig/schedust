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

use std::{fs::File, time::Duration};

use scheduler::Processes;

use process::Process;
use task::{Email, ParallelTask, Print};

use crate::task::{PrintVariable, SetVariable};
// use task::Task;
// use once_cell::sync::Lazy;

mod process;
mod scheduler;
mod server;
mod task;

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

    // Create a new task
    let process1 = Process::new(
        "process1".to_string(),
        "1/10 * * * * *".into(),
        vec![
            // Script::new_task(
            //     r"C:\Users\jacob\Desktop\rusttres\target\release\rusttres.exe".into(),
            //     r"C:\Users\jacob\Desktop\rusttres".into(),
            // ),
            Email::new_task(
                vec!["abc@gmail.com".into(), "admin@admin.com".into()],
                "ME".into(),
                "Test Subject!".into(),
                "Some Optional conditional message".into(),
            ),
            Print::new_task(r"Completed Process1!".into()),
        ],
    );

    let process2 = Process::new(
        "process2".to_string(),
        "1/3 * * * * *".into(),
        vec![
            SetVariable::new_task("money!".into(), "No!".into()),
            ParallelTask::new_task(vec![
                Print::new_task("Hello".into()),
                Print::new_task("World".into()),
                Print::new_task("1".into()),
                Print::new_task("2".into()),
                Print::new_task("3".into()),
                Print::new_task("4".into()),
                Print::new_task("5".into()),
                Print::new_task("6".into()),
                Print::new_task("7".into()),
                Print::new_task("8".into()),
                Print::new_task("9".into()),
            ]),
            SetVariable::new_task("money!".into(), "Yeah!".into()),
            // Script::new_task(
            //     r"C:\Users\jacob\Desktop\rusttres\target\release\rusttres.exe".into(),
            //     r"C:\Users\jacob\Desktop\rusttres".into(),
            // ),
            // Script::new_task(
            //     r"C:\Users\jacob\Desktop\rusttres\target\release\rusttres.exe".into(),
            //     r"C:\Users\jacob\Desktop\rusttres".into(),
            // ),
            PrintVariable::new_task("money!".into()),
            Print::new_task(r"Completed Process2!".into()),
        ],
    );

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
