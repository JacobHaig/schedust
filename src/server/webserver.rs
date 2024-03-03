use crate::server::scheduler::Processes;

use warp::Filter;

pub async fn start(processes: Processes) {
    // Main directory
    let processes_clone = processes.clone();
    let root =
        warp::path::end().map(move || serde_json::to_string_pretty(&processes_clone).unwrap());

    // Example of a path
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let routes = root.or(hello);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
