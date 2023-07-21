use std::sync::{Arc, Mutex};

use crate::process::Process;

pub type Processes = Arc<Mutex<Vec<Arc<Mutex<Process>>>>>;
pub struct Scheduler<'a> {
    pub processes: Processes,
    pub scheduler: job_scheduler::JobScheduler<'a>,
}

impl<'a> Scheduler<'a> {
    pub async fn new() -> Scheduler<'a> {
        Scheduler {
            processes: Arc::new(Mutex::new(Vec::new())),
            scheduler: job_scheduler::JobScheduler::new(),
        }
    }

    pub async fn add_process(&mut self, process: Arc<Mutex<Process>>) {
        let time_clone = process.lock().unwrap().timing_cron.clone();
        let process_clone = process.clone();

        let job = job_scheduler::Job::new(time_clone.parse().unwrap(), move || {
            process_clone.lock().unwrap().run();
        });

        self.scheduler.add(job);

        self.processes.lock().unwrap().push(process);
    }

    pub async fn run_once(&mut self) {
        self.scheduler.tick();
    }

    pub async fn get_process_list(&self) -> Processes {
        self.processes.clone()
    }
}
