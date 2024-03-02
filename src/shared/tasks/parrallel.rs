use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::shared::tasks::task::Task;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParallelTask {
    pub tasks: Vec<Arc<Mutex<Task>>>,
}

impl ParallelTask {
    pub fn new(tasks: Vec<Arc<Mutex<Task>>>) -> Self {
        Self { tasks }
    }
    pub fn to_ref(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn to_task(self) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Parallel(self)))
    }

    pub fn run(&self) {
        // ParallelTask spins up its own threads
        // and dispatches tasks to them
        let mut handles = vec![];

        for task in &self.tasks {
            let task_c = task.clone();
            // let variables_c = variables.clone();

            let handle = std::thread::spawn(move || {
                let task_locked = task_c.lock().unwrap();

                task_locked.run_task();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
