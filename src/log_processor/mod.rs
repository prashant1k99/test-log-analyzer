use std::collections::HashMap;

use crate::{
    config::{Config, Target},
    errors::LogAnalyzerErrors,
};

mod parallel;
mod sequential;

pub use parallel::ParallelLogProcessor;
pub use sequential::SequentialLogProcessor;

pub struct Summary {
    target: Target,
    records: HashMap<String, u64>,
}

impl Summary {
    pub fn print(&self) {
        println!("\n== Final Counts (By {:?}) ==", self.target);
        for (key, count) in &self.records {
            println!("{}: {}", key, count);
        }
        println!("=============================");
    }
}

pub trait LogProcessor {
    fn process(&self) -> Result<Summary, LogAnalyzerErrors<'_>>;
}

pub struct LogEntry<'a> {
    level: &'a str,
    _time: &'a str,
    service: &'a str,
    _message: &'a str,
}

pub fn process_log_line<'a>(cfg: &Config, line: &'a str) -> Result<LogEntry<'a>, String> {
    let content: Vec<&str> = line.split(&cfg.delimiter).collect();

    if content.len() != 4 {
        return Err("Malformed".to_string());
    }

    let level = if cfg.levels.contains(&content[1].to_string()) {
        content[1]
    } else {
        "UNKNOWN"
    };

    Ok(LogEntry {
        _time: content[0],
        level,
        service: content[2],
        _message: content[3],
    })
}
