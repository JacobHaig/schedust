use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::task::Task;

#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,

    pub agent: String,
    pub task: Task,
}

impl Job {
    pub fn new(name: &str, description: &str, agent: &str, task: Task) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            agent: agent.into(),
            task,
        }
    }

    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    // Runs the job
    // Have this be the starting point to send the job to the agent
    // so that the agent can run the job
    pub fn run(&self) {
        self.task.run();
    }
}
