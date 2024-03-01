# SCHEDUST

SCHEDUST is a simple scheduling tool for the command line. It is designed to be simple and easy to use, and to be able to handle a variety of scheduling needs. It is written in Rust and is configured using a simple YAML file. It is designed to be easy to use and to be able to handle a variety of scheduling needs.

## Building and Running

Rust is required to build SCHEDUST. If you do not have Rust installed, you can install it using [rustup](https://rustup.rs/).

SCHEDUST is built using Cargo. To build it, simply run the following command in the root directory of the project:

```bash
cargo run --release
```

## Configuration
From here, you can modify the `task.yaml` file to suit your needs. This is not well documented yet, but the file is simple and should be easy to understand. Each Process is an independent task that runs other tasks on a schedule. They are defined in the `task.yaml` file. Here is an example of a simple task:

```yaml
- name: process1
    id: 1f450f51-7633-4928-9a29-449317169e6a
    timing_cron: 1/3 * * * * *
    task: 
        !Sequential
            tasks:
            - !Script
                command: "C:\\Users\\USER\\random_exe.exe"
                env: "C:\\Users\\USER\\Documents"
            - !Print
                message: Running Executable!
```

You can see that the timing is defined using a cron string. This also you to quickly setup recurring tasks easily. 



## Todo
One of the major tasks to complete is to add support for agents. This will allow SCHEDUST to run tasks on remote machines. This will be a major feature and will require a bunch of work to implement. 