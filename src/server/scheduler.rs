use std::sync::{Arc, Mutex};

use crate::shared::process::Process;

pub type Processes = Vec<Arc<Mutex<Process>>>;
pub struct Scheduler<'a> {
    pub processes: Processes,
    pub scheduler: job_scheduler_ng::JobScheduler<'a>,
}

impl<'a> Scheduler<'a> {
    pub async fn new() -> Scheduler<'a> {
        Scheduler {
            processes: Vec::new(),
            scheduler: job_scheduler_ng::JobScheduler::new(),
        }
    }

    pub async fn add_process(&mut self, process: Arc<Mutex<Process>>) {
        let time_clone = process.lock().unwrap().timing_cron.clone();
        let process_clone1 = process.clone();

        let job = job_scheduler_ng::Job::new(time_clone.parse().unwrap(), move || {
            let process_clone2 = process_clone1.clone();

            // Run each process, when the time is right, in a new thread.
            std::thread::spawn(move || {
                process_clone2.clone().lock().unwrap().run();
            });
        });

        self.scheduler.add(job);

        self.processes.push(process);
    }

    pub async fn run_once(&mut self) {
        self.scheduler.tick();
    }

    pub async fn run(&mut self) {
        loop {
            self.run_once().await;
            let time = self.scheduler.time_till_next_job();
            tokio::time::sleep(time).await;
        }
    }

    pub async fn get_process_list(&self) -> Processes {
        self.processes.clone()
    }
}
