use serde::{Deserialize, Serialize};

use super::process::Process;

#[derive(Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub name: String,
    pub jobs: Vec<Process>,
}

impl Sheet {
    pub fn new(name: String, jobs: Vec<Process>) -> Self {
        Sheet { name, jobs }
    }
}
