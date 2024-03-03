use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::shared::task::Task;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequentialTask {
    pub tasks: Vec<Arc<Mutex<Task>>>,
}

impl SequentialTask {
    pub fn new(tasks: Vec<Arc<Mutex<Task>>>) -> Self {
        Self { tasks }
    }
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
    pub fn to_task(self) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Sequential(self)))
    }
    pub fn run(&self) {
        for task in &self.tasks {
            let task_c = task.clone();
            // let variables_c = variables.clone();

            let task_locked = task_c.lock().unwrap();

            task_locked.run_task();
        }
    }
}
