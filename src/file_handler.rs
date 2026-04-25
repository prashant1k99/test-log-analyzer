use std::path::Path;

use crate::errors::LogAnalyzerErrors;

pub struct FileHandler<'a>(pub &'a String);

impl<'a> FileHandler<'a> {
    pub fn validate(&self) -> Result<(), LogAnalyzerErrors<'_>> {
        if Path::new(&self.0).exists() {
            Ok(())
        } else {
            Err(LogAnalyzerErrors::FileNotFound(self.0))
        }
    }
}
