use serde::{Deserialize, Serialize};
use std::{
    default,
    process::Command,
    sync::{Arc, Mutex},
};

use crate::shared::task::Task;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]

pub struct ScriptTask {
    command: String,
    env: String,

    description: Option<String>,
}

impl ScriptTask {
    pub fn new(command: String, env: String) -> Self {
        ScriptTask {
            command,
            env,
            ..default::Default::default()
        }
    }
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn to_task(self) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Script(self)))
    }

    pub fn run(&self) {
        let result = Command::new(&self.command)
            // .arg("-c")
            // .arg(&self.command)
            .current_dir(&self.env)
            .output();

        match result {
            Ok(output) => {
                println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
