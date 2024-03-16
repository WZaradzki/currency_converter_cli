use std::io::Error;

use serde::{Deserialize, Serialize};

use crate::cache::{
    file_cache::{create_cache_file, read_and_invalid_cache_file},
    CacheConfigs,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandHistory {
    command: String,
    output: String,
    timestamp: String,
}

impl CommandHistory {
    pub fn new(command: String, output: String) -> CommandHistory {
        let timestamp = chrono::Utc::now().to_rfc2822();

        CommandHistory {
            command,
            output,
            timestamp,
        }
    }

    pub fn get_commands() -> Result<Vec<CommandHistory>, String> {
        let history: Result<Vec<CommandHistory>, Error> =
            read_and_invalid_cache_file(CacheConfigs::CommandHistory, None);

        match history {
            Ok(history) => Ok(history),
            Err(e) => Err(e.to_string()),
        }
    }
    pub fn save(output: String, command: String) -> Result<(), String> {
        let command = CommandHistory::new(command, output);
        let history = CommandHistory::get_commands();

        let history: Vec<CommandHistory> = match history {
            Ok(mut history) => {
                history.push(command);
                history
            }
            Err(_) => {
                vec![command]
            }
        };

        let result = create_cache_file(&history, CacheConfigs::CommandHistory, None);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_output(&self) -> &String {
        &self.output
    }

    pub fn get_timestamp(&self) -> &String {
        &self.timestamp
    }
}

pub async fn get_history() -> Result<Vec<CommandHistory>, String> {
    let history = CommandHistory::get_commands();

    match history {
        Ok(history) => Ok(history),
        Err(e) => Err(e),
    }
}
