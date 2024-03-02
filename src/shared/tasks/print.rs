use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::shared::tasks::task::Task;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrintTask {
    pub message: String,
}

impl PrintTask {
    pub fn new(message: &str) -> Self {
        PrintTask {
            message: message.into(),
        }
    }
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
    pub fn to_task(self) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Print(self)))
    }
    pub fn run(&self) {
        println!("{}", self.message);
    }
}
