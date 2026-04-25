use std::env;

use test_log_analyzer::{errors::LogAnalyzerErrors, file_handler::FileHandler};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return LogAnalyzerErrors::LogFileNotProvided.out();
    }

    let file_handler = FileHandler(&args[1]);

    if let Err(e) = file_handler.validate() {
        return e.out();
    }

    println!("Process File");
}
