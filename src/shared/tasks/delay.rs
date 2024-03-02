use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::shared::tasks::task::Task;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DelayTask {
    pub delay: u64,
}

impl DelayTask {
    pub fn new(delay: u64) -> Self {
        DelayTask { delay }
    }

    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
    pub fn to_task(self) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Delay(self)))
    }
    pub fn run(&self) {
        std::thread::sleep(Duration::from_secs(self.delay));
    }
}
