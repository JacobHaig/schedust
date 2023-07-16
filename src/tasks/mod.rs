use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub mod email;
pub mod parrallel;
pub mod print;
pub mod script;
pub mod sequential;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Task {
    Print(print::PrintTask),
    Script(script::ScriptTask),
    Email(email::EmailTask),
    Parallel(parrallel::ParallelTask),
    Sequential(sequential::SequentialTask),
}

pub trait TaskTrait {
    fn run(&self);

    fn get_name(&self) -> String;

    fn get_timing(&self) -> String;

    fn get_output(&self) -> String;
}

impl Task {
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn run(&self) {
        self.get_inner_task().run();
    }

    pub fn get_inner_task(&self) -> &dyn TaskTrait {
        match self {
            Task::Print(task) => task,
            Task::Script(task) => task,
            Task::Email(task) => task,
            Task::Parallel(task) => task,
            Task::Sequential(task) => task,
        }
    }
}
