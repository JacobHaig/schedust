use serde::{Deserialize, Serialize};
use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use super::Task;

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct ScriptTask {
    command: String,
    env: String,
}

impl ScriptTask {
    pub fn new(command: String, env: String) -> Self {
        ScriptTask { command, env }
    }
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn to_task(self) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Script(self)))
    }

    pub fn run(&self) {
        Command::new(&self.command)
            // .arg("-c")
            // .arg(&self.command)
            .current_dir(&self.env)
            .output()
            .expect("failed to execute process");
    }
}
