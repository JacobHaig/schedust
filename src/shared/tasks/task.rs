use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::shared::tasks::{
    delay::DelayTask, email::EmailTask, parrallel::ParallelTask, print::PrintTask,
    script::ScriptTask, sequential::SequentialTask,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// #[serde(tag = "type")]

pub enum Task {
    Print(PrintTask),
    Script(ScriptTask),
    Email(EmailTask),
    Parallel(ParallelTask),
    Sequential(SequentialTask),
    Delay(DelayTask),
}

impl Task {
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn run(&self) {
        self.run_task();
    }

    pub fn run_task(&self) {
        match self {
            Task::Print(task) => task.run(),
            Task::Script(task) => task.run(),
            Task::Email(task) => task.run(),
            Task::Parallel(task) => task.run(),
            Task::Sequential(task) => task.run(),
            Task::Delay(task) => task.run(),
        }
    }
}
