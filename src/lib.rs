pub mod shared {
    pub mod job;
    pub mod process;
    pub mod sheet;
    pub mod task;
    pub mod tasks {
        pub mod delay;
        pub mod email;
        pub mod parrallel;
        pub mod print;
        pub mod script;
        pub mod sequential;
    }
}

pub mod server {
    pub mod scheduler;
    pub mod webserver;
}
