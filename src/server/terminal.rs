use once_cell::sync::Lazy;

pub static mut RUNNING: Lazy<bool> = Lazy::new(|| true);

pub struct Terminal {
    // sx: tokio::sync::mpsc::Sender<String>,
    // rx: tokio::sync::mpsc::Receiver<String>,
}

impl Terminal {
    // pub fn new() -> Terminal {
    //     let (sx, rx) = tokio::sync::mpsc::channel(1);

    //     Terminal { sx, rx }
    // }

    pub async fn start(mut rx: tokio::sync::mpsc::Receiver<String>) {
        ctrlc::set_handler(|| unsafe {
            *RUNNING = false;
        })
        .expect("Error setting Ctrl-C handler");

        // Keep running until Ctrl-C is pressed
        while unsafe { *crate::terminal::RUNNING } {
            let str_future = Terminal::read_input_line();

            let line_future = rx.recv();

            let output = tokio::select! {
                line = line_future => line.unwrap(),
                str = str_future => str
            };

            println!("{}!!!!!!!!!!!!!!!!", output);
        }
    }

    // Read a line from the terminal when the user presses enter
    pub async fn read_input_line() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
