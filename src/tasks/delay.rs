use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::Task;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DelayTask {
    pub delay: Duration,
}

impl DelayTask {
    pub fn new(delay: Duration) -> Self {
        DelayTask {
            delay: delay.into(),
        }
    }

    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
    pub fn to_task(self) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Delay(self)))
    }
    pub fn run(&self) {
        std::thread::sleep(self.delay);
    }
}
