pub enum LogAnalyzerErrors<'a> {
    LogFileNotProvided,
    FileNotFound(&'a String),
}

impl<'a> LogAnalyzerErrors<'a> {
    pub fn out(&self) {
        println!("======");
        println!("ERROR:");
        match self {
            LogAnalyzerErrors::LogFileNotProvided => {
                println!("Log file path not provided, please provide file path")
            }
            LogAnalyzerErrors::FileNotFound(file) => {
                println!("File Does Not Exists. Invalid file path `{}`", file)
            }
        };
        println!("======");
    }
}
