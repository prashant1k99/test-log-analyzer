use std::sync::Arc;

use crate::{
    config::Config,
    errors::LogAnalyzerErrors,
    log_processor::{LogProcessor, Summary},
};

pub struct ParallelLogProcessor<'a> {
    pub file_path: &'a String,
    pub cfg: Arc<Config>,
}

impl<'a> LogProcessor for ParallelLogProcessor<'a> {
    fn process(&self) -> Result<Summary, LogAnalyzerErrors<'_>> {
        todo!()
    }
}
