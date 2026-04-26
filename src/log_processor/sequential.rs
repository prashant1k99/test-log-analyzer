use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

use crate::{
    config::{Config, Target},
    errors::LogAnalyzerErrors,
    log_processor::{LogProcessor, Summary, process_log_line},
};

pub struct SequentialLogProcessor<'a> {
    pub file_path: &'a String,
    pub cfg: Arc<Config>,
}

impl<'a> LogProcessor for SequentialLogProcessor<'a> {
    fn process(&self) -> Result<Summary, LogAnalyzerErrors<'_>> {
        let file = File::open(self.file_path).map_err(|err| {
            LogAnalyzerErrors::IoError(self.file_path, format!("Unable to read log file {err}"))
        })?;

        let mut counts: HashMap<String, u64> = HashMap::new();

        for line in BufReader::new(file).lines() {
            let line = line.map_err(|err| {
                LogAnalyzerErrors::IoError(self.file_path, format!("Unable to read log line {err}"))
            })?;

            let res = process_log_line(self.cfg.clone(), &line);

            match res {
                Ok(log_entry) => match self.cfg.target {
                    Target::Level => {
                        *counts.entry(log_entry.level.to_string()).or_insert(0) += 1;
                    }
                    Target::Service => {
                        *counts.entry(log_entry.service.to_string()).or_insert(0) += 1;
                    }
                },
                Err(_) => *counts.entry("MALFORMED".to_string()).or_insert(0) += 1,
            }
        }

        Ok(Summary {
            target: self.cfg.target.clone(),
            records: counts,
        })
    }
}
