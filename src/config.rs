use std::fs;

use serde::Deserialize;

use crate::errors::LogAnalyzerErrors;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Level,
    Service,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub delimiter: String,
    pub levels: Vec<String>,
    pub parallel: Option<bool>,
    pub target: Target,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delimiter: String::from("|"),
            levels: vec!["INFO".to_string(), "WARN".to_string(), "ERROR".to_string()],
            parallel: None,
            target: Target::Level,
        }
    }
}

impl Config {
    pub fn read_from_file<'a>(file_path: &'a String) -> Result<Self, LogAnalyzerErrors<'a>> {
        let config_content = fs::read_to_string(file_path).map_err(|err| {
            LogAnalyzerErrors::ConfigReadError(
                format!("Failed to read config file `{}`", err),
                file_path,
            )
        })?;
        let config: Config = toml::from_str(&config_content).map_err(|err| {
            LogAnalyzerErrors::ConfigReadError(
                format!("Failed to parse config file `{}`", err),
                file_path,
            )
        })?;

        Ok(config)
    }
}
