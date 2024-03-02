// extern crate schedust;

// mod crate::shared::process;

// mod shared::process;
// mod tasks;

mod scheduler;
mod shared;
mod webserver;

use std::fs::File;

use crate::shared::process::Process;
use crate::shared::tasks::{
    delay::DelayTask, email::EmailTask, parrallel::ParallelTask, print::PrintTask,
    script::ScriptTask, sequential::SequentialTask, Task,
};
use scheduler::Processes;

#[tokio::main]
async fn main() {
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
            ParallelTask::new(vec![
                PrintTask::new("1").to_task(),
                DelayTask::new(1).to_task(),
                PrintTask::new("2").to_task(),
                DelayTask::new(1).to_task(),
                PrintTask::new("3").to_task(),
                DelayTask::new(1).to_task(),
                PrintTask::new("4").to_task(),
                DelayTask::new(1).to_task(),
                PrintTask::new("5").to_task(),
                DelayTask::new(1).to_task(),
                PrintTask::new("6").to_task(),
                DelayTask::new(1).to_task(),
                PrintTask::new("7").to_task(),
                DelayTask::new(1).to_task(),
                PrintTask::new("8").to_task(),
                DelayTask::new(1).to_task(),
                PrintTask::new("9").to_task(),
            ])
            .to_task(),
            PrintTask::new("Completed Process2!").to_task(),
        ])
        .to_task(),
    )
    .to_ref();

    use std::io::Write;
    let s = serde_yaml::to_string(&process2).unwrap();
    let mut file = File::create("results.yaml").unwrap();
    write!(file, "{}", s).unwrap();

    let processes_str = std::fs::read_to_string("task.yaml").unwrap();
    let processes: Vec<Process> = serde_yaml::from_str(&processes_str).unwrap();

    // Create a new scheduler
    let mut scheduler = scheduler::Scheduler::new().await;

    // Add the task to the scheduler
    for process in processes {
        scheduler.add_process(process.to_ref()).await;
    }

    // Run the web-server
    let process_list: Processes = scheduler.get_process_list().await;
    tokio::task::spawn(webserver::start(process_list.clone()));

    // for _ in 0..4 {
    //     scheduler.run_once().await;
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    // }

    // Run indefinitely
    scheduler.run().await;
}
