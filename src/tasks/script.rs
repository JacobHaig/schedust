use crate::tasks::TaskTrait;
use serde::{Deserialize, Serialize};
use std::{
    process::Command,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScriptTask {
    command: String,
    env: String,
}

impl ScriptTask {
    pub fn new(command: String, env: String) -> Self {
        ScriptTask {
            command,
            env,
            // ..Default::default()
        }
    }
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}

impl TaskTrait for ScriptTask {
    fn run(&self) {
        Command::new(&self.command)
            // .arg("-c")
            // .arg(&self.command)
            .current_dir(&self.env)
            .output()
            .expect("failed to execute process");
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_timing(&self) -> String {
        todo!()
    }

    fn get_output(&self) -> String {
        todo!()
    }
}
