pub mod shared {
    pub mod process;
    pub mod tasks {
        pub mod delay;
        pub mod email;
        pub mod parrallel;
        pub mod print;
        pub mod script;
        pub mod sequential;
        pub mod task;
    }
}

pub mod server {
    pub mod scheduler;
    pub mod webserver;
}
