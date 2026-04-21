/// SHOULD REMOVE THIS STATE BECAUSE NOW IT'S A FIELD IN EACH TASK
use std::{
    fs::{self, File},
    io::Write,
};

use crate::task::{DefaultState, Task};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("Failed to read state file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to serialize state: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("Failed to parse state file: {0}")]
    Deserialize(#[from] toml::de::Error),
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub tasks: Vec<Task>,
}

impl State {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn get_state(&self, state: DefaultState) -> Vec<Task> {
        self.tasks
            .iter()
            .filter(|task| task.state == state)
            .cloned()
            .collect()
    }

    pub fn write_state(&self, file_path: &str) -> Result<(), StateError> {
        let res = toml::to_string(&self)?;
        let mut file = File::create(file_path)?;

        file.write_all(res.as_bytes())?;

        Ok(())
    }

    pub fn load_state(&mut self, file_path: &str) -> Result<(), StateError> {
        let file_content = fs::read_to_string(file_path)?;

        // Parse the file content into a State struct,
        // or create a new empty State if the file is empty
        if file_content.trim().is_empty() {
            self.tasks = vec![];
        } else {
            let parsed = toml::from_str::<State>(&file_content)?;

            self.tasks = parsed.tasks;
        };

        Ok(())
    }
}
