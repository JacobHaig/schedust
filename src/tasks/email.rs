use std::sync::{Arc, Mutex};

use crate::tasks::TaskTrait;

use serde::{Deserialize, Serialize};

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
}

impl TaskTrait for EmailTask {
    fn run(&self) {
        println!("Sending email to {:?}", self.to);
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
