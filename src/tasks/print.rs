use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::tasks::TaskTrait;

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
}

impl TaskTrait for PrintTask {
    fn run(&self) {
        println!("{}", self.message);
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
