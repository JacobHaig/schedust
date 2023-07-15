use std::{process::Command, sync::Arc};

// use tokio::sync::Mutex;
use crate::process::Variables;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Bunch of possible tasks
// https://docs.asg.com/op_znh_4.2/znzug/definingtasks/defining_tasks.htm
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Task {
    Print(Print),
    Script(Script),
    Email(Email),
    // FileOperation(FileOperation),
    // HTTP(HTTP),
    IfCondition(IfCondition),
    SetVariable(SetVariable),
    PrintVariable(PrintVariable),

    ParallelTask(ParallelTask),
    // SubTask(SubTask),
}

impl Task {
    pub fn run(&self, variables: Variables) {
        match &*self {
            Task::Print(task) => task.run(variables),
            Task::Script(task) => task.run(variables),
            Task::Email(task) => task.run(variables),
            Task::IfCondition(task) => task.run(variables),
            Task::ParallelTask(task) => task.run(variables),
            Task::SetVariable(task) => task.run(variables),
            Task::PrintVariable(task) => task.run(variables),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Print {
    pub message: String,
}

impl Print {
    pub fn new(message: String) -> Print {
        Print { message }
    }
    pub fn new_task(text: String) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Print(Print::new(text))))
    }
    pub fn run(&self, _variables: Variables) {
        println!("{}", self.message);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Email {
    pub to: Vec<String>,
    pub from: String,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn new(to: Vec<String>, from: String, subject: String, body: String) -> Email {
        Email {
            to,
            from,
            subject,
            body,
        }
    }

    pub fn new_task(
        to: Vec<String>,
        from: String,
        subject: String,
        body: String,
    ) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Email(Email::new(to, from, subject, body))))
    }

    pub fn run(&self, _variables: Variables) {
        println!("Sending email to {:?}", self.to);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    command: String,
    env: String,
}

impl Script {
    pub fn new(command: String, env: String) -> Script {
        Script {
            command,
            env,
            // ..Default::default()
        }
    }

    // This essentially just wraps the `Script` in a `Task`
    pub fn new_task(command: String, env: String) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::Script(Script::new(command, env))))
    }

    pub fn run(&self, _variables: Variables) {
        Command::new(&self.command)
            // .arg("-c")
            // .arg(&self.command)
            .current_dir(&self.env)
            .output()
            .expect("failed to execute process");
    }
}

// pub enum ConditionType {
//     WaitForCompletion()
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IfCondition {}

impl IfCondition {
    // pub fn new(tasks: Vec<Arc<Mutex<Task>>>) -> ParallelTask {
    //     ParallelTask { tasks }
    // }
    // pub fn new_task(tasks: Vec<Arc<Mutex<Task>>>) -> Arc<Mutex<Task>> {
    //     Arc::new(Mutex::new(Task::ParallelTask(ParallelTask::new(tasks))))
    // }

    pub fn run(&self, variables: Variables) {
        // ParallelTask spins up its own threads
        // and dispatches tasks to them
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParallelTask {
    pub tasks: Vec<Arc<Mutex<Task>>>,
    // pub tasks: Vec<Task>,
}

impl ParallelTask {
    pub fn new(tasks: Vec<Arc<Mutex<Task>>>) -> ParallelTask {
        ParallelTask { tasks }
    }
    pub fn new_task(tasks: Vec<Arc<Mutex<Task>>>) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::ParallelTask(ParallelTask::new(tasks))))
    }

    pub fn run(&self, variables: Variables) {
        // ParallelTask spins up its own threads
        // and dispatches tasks to them
        let mut handles = vec![];

        for task in &self.tasks {
            let task_c = task.clone();
            let variables_c = variables.clone();

            let handle = std::thread::spawn(move || {
                let task_unlocked = task_c.lock().unwrap();

                task_unlocked.run(variables_c);
                // match &*task_unlocked {
                //     Task::Print(task) => task.run(variables_c),
                //     Task::Script(task) => task.run(variables_c),
                //     Task::Email(task) => task.run(variables_c),
                //     Task::IfCondition(task) => task.run(variables_c),
                //     Task::ParallelTask(task) => task.run(variables_c),
                //     Task::SetVariable(task) => task.run(variables_c),
                //     Task::PrintVariable(task) => task.run(variables_c),
                // }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetVariable {
    pub name: String,
    pub value: String,
}

impl SetVariable {
    pub fn new(name: String, value: String) -> SetVariable {
        SetVariable { name, value }
    }
    pub fn new_task(name: String, value: String) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::SetVariable(SetVariable::new(name, value))))
    }

    pub fn run(&self, variables: Variables) {
        let mut vari = variables.lock().unwrap();
        vari.insert(self.name.clone(), self.value.clone());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrintVariable {
    pub name: String,
}

impl PrintVariable {
    pub fn new(name: String) -> PrintVariable {
        PrintVariable { name }
    }
    pub fn new_task(name: String) -> Arc<Mutex<Task>> {
        Arc::new(Mutex::new(Task::PrintVariable(PrintVariable::new(name))))
    }

    pub fn run(&self, variables: Variables) {
        let vari = variables.lock().unwrap();
        println!("{}: {}", self.name, vari.get(&self.name).unwrap());
    }
}
