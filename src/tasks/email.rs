use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use super::Task;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailTask {
    pub to: Vec<String>,
    pub from: String,
    pub subject: String,
    pub body: String,
}

impl EmailTask {
    pub fn new(to: Vec<&str>, from: &str, subject: &str, body: &str) -> EmailTask {
        EmailTask {
            to: to.into_iter().map(String::from).collect(),
            from: from.into(),
            subject: subject.into(),
            body: body.into(),
        }
    }
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
    pub fn to_task(self) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Email(self)))
    }
    pub fn run(&self) {
        println!("[MOCK] Sending email to {:?}", self.to);
    }
}
