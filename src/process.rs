use crate::task::Task;
// use crate::task::Script;
// use crate::task::Email;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;

use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
    Active,
    Completed(String),
    Error(String),
    Aborted,
    Cancelled,
    Hold,
    Scheduled,
}

pub type Variables = Arc<Mutex<std::collections::HashMap<String, String>>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Process {
    pub name: String,
    id: uuid::Uuid,
    // pub task: Task,
    pub current_status: Status,
    pub status_history: Vec<Status>,

    // The timing is in CRON format
    pub timing: String,
    pub variables: Variables,

    pub tasks: Vec<Arc<Mutex<Task>>>,
}

// implementation for Task
impl Process {
    pub fn new(name: String, timing: String, tasks: Vec<Arc<Mutex<Task>>>) -> Arc<Mutex<Process>> {
        Arc::new(Mutex::new(Process {
            name,
            id: Uuid::nil(), // empty UUID. This gets set by the scheduler

            current_status: Status::Scheduled,
            status_history: Vec::new(),
            variables: Arc::new(Mutex::new(std::collections::HashMap::new())),

            timing,
            tasks,
        }))
    }

    // pub fn set_next(&mut self, next_task: Arc<Mutex<Process>>) {
    //     self.tasks = Some(next_task);
    // }

    pub fn run(&mut self) {
        println!("Running task: {}", self.name);
        // self.current_status = crate::task::Status::ACTIVE;

        // let output = self.command.run();

        // This should be done in a separate thread
        for task in self.tasks.clone() {
            let task_lock = task.lock().unwrap();

            task_lock.run(self.variables.clone());
            // match &*task_lock {
            //     Task::Print(task) => task.run(self.variables.clone()),
            //     Task::Script(task) => task.run(self.variables.clone()),
            //     Task::Email(task) => task.run(self.variables.clone()),
            //     Task::IfCondition(task) => task.run(self.variables.clone()),
            //     Task::ParallelTask(task) => task.run(self.variables.clone()),
            //     Task::SetVariable(task) => task.run(self.variables.clone()),
            //     Task::PrintVariable(task) => task.run(self.variables.clone()),
            // }
        }

        // Update the status of the process

        // if output.status.success() {
        //     self.current_status = Status::COMPLETED(String::from_utf8_lossy(&output.stdout).into());
        // } else {
        //     self.current_status = Status::ERROR(String::from_utf8_lossy(&output.stderr).into());
        // }

        // println!("status: {}", output.status.success());
        // println!("stdout : {}", String::from_utf8_lossy(&output.stdout));
        // println!("stderr : {}", String::from_utf8_lossy(&output.stderr));
    }

    /// Set the task's id.
    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    /// Get the task's id.
    #[must_use]
    pub fn _get_id(&self) -> Uuid {
        self.id
    }
}
