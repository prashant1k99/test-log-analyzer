use std::{fs::File, path::Path};

use crate::errors::LogAnalyzerErrors;

pub struct FileHandler<'a>(pub &'a String);

impl<'a> FileHandler<'a> {
    pub fn validate(&self) -> Result<(), LogAnalyzerErrors<'_>> {
        if Path::new(&self.0).exists() {
            Ok(())
        } else {
            Err(LogAnalyzerErrors::FileNotFound(&self.0))
        }
    }
    pub fn file_size(&self) -> Result<u64, LogAnalyzerErrors<'_>> {
        let file = File::open(&self.0).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => LogAnalyzerErrors::FileNotFound(&self.0),
            std::io::ErrorKind::PermissionDenied => LogAnalyzerErrors::PermissionDenied(&self.0),
            _ => LogAnalyzerErrors::IoError(self.0, "Unable to fetch file".to_string()),
        })?;

        let metadata = file.metadata().map_err(|_| {
            LogAnalyzerErrors::IoError(&self.0, "Unable to read metadata".to_string())
        })?;
        Ok(metadata.len())
    }
}
