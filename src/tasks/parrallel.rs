use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::tasks::Task;
use crate::tasks::TaskTrait;

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
}

impl TaskTrait for ParallelTask {
    fn run(&self) {
        // ParallelTask spins up its own threads
        // and dispatches tasks to them
        let mut handles = vec![];

        for task in &self.tasks {
            let task_c = task.clone();
            // let variables_c = variables.clone();

            let handle = std::thread::spawn(move || {
                let task_locked = task_c.lock().unwrap();

                task_locked.get_inner_task().run();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
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
