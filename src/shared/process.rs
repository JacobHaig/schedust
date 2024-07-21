use crate::shared::task::Task;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::job::Job;

// #[derive(Serialize, Deserialize, Clone)]
// pub enum Status {
//     Active,
//     Completed(String),
//     Error(String),
//     Aborted,
//     Cancelled,
//     Hold,
//     Scheduled,
// }

pub type Variables = Arc<Mutex<HashMap<String, String>>>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Process {
    id: uuid::Uuid,
    pub name: String,
    // pub current_status: Status,
    // pub status_history: Vec<Status>,

    // The timing is in CRON format
    pub timing_cron: String,
    pub variables: Variables,

    // pub job: Arc<Mutex<Job>>,
    pub job: Job,
}

// implementation for Task
impl Process {
    // pub fn new(name: &str, timing: &str, job: Arc<Mutex<Job>>) -> Process {
    pub fn new(name: &str, timing: &str, job: Job) -> Process {
        Process {
            name: name.into(),
            id: Uuid::nil(), // empty UUID. This gets set by the scheduler

            // current_status: Status::Scheduled,
            // status_history: Vec::new(),
            // agent: agent.into(),

            // Second, Minute, Hour, Day of Month, Month, Day of Week
            timing_cron: timing.into(),
            variables: Arc::new(Mutex::new(HashMap::new())),

            job,
        }
    }

    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    // Run the process to tigger the `job flow`.
    pub fn run(&mut self) {
        self.job.clone().lock().unwrap().run();
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
