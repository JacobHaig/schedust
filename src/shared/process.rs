use crate::shared::task::Task;

use std::sync::Arc;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Process {
    pub name: String,
    id: uuid::Uuid,
    // pub task: Task,
    pub current_status: Status,
    pub status_history: Vec<Status>,

    pub agent: String,

    // The timing is in CRON format
    pub timing_cron: String,
    pub variables: Variables,

    pub task: Arc<Mutex<Task>>,
}

// implementation for Task
impl Process {
    pub fn new(name: &str, timing: &str, agent: &str, task: Arc<Mutex<Task>>) -> Process {
        Process {
            name: name.into(),
            id: Uuid::nil(), // empty UUID. This gets set by the scheduler

            current_status: Status::Scheduled,
            status_history: Vec::new(),

            agent: agent.into(),

            // Second, Minute, Hour, Day of Month, Month, Day of Week
            timing_cron: timing.into(),
            variables: Arc::new(Mutex::new(std::collections::HashMap::new())),

            task,
        }
    }

    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    // pub fn set_next(&mut self, next_task: Arc<Mutex<Process>>) {
    //     self.tasks = Some(next_task);
    // }

    pub fn run(&mut self) {
        self.task.clone().lock().unwrap().run();
    }

    /// Set the task's id.
    pub fn _set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    /// Get the task's id.
    #[must_use]
    pub fn _get_id(&self) -> Uuid {
        self.id
    }
}
