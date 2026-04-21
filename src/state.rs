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
    pub todo: Vec<Task>,
    pub done: Vec<Task>,
    pub doing: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct FileState {
    pub tasks: Vec<Task>,
}

impl State {
    pub fn new() -> Self {
        Self {
            todo: vec![],
            done: vec![],
            doing: vec![],
        }
    }

    pub fn write_state(&self, file_path: &str) -> Result<(), StateError> {
        let mut file_state = FileState { tasks: vec![] };

        let mut todos = self.todo.clone();
        let mut doings = self.doing.clone();
        let mut dones = self.done.clone();

        file_state.tasks.append(&mut todos);
        file_state.tasks.append(&mut doings);
        file_state.tasks.append(&mut dones);

        let res = toml::to_string(&file_state)?;
        let mut file = File::create(file_path)?;

        file.write_all(res.as_bytes())?;

        Ok(())
    }

    pub fn load_state(&mut self, file_path: &str) -> Result<(), StateError> {
        let file_content = fs::read_to_string(file_path)?;

        // Parse the file content into a State struct,
        // or create a new empty State if the file is empty
        if file_content.trim().is_empty() {
            self.todo = vec![];
            self.done = vec![];
            self.doing = vec![];
        } else {
            let file_state = toml::from_str::<FileState>(&file_content)?;

            self.doing = filter_for_state(&file_state.tasks, DefaultState::Doing);
            self.done = filter_for_state(&file_state.tasks, DefaultState::Done);
            self.todo = filter_for_state(&file_state.tasks, DefaultState::Todo);
        };

        Ok(())
    }
}

fn filter_for_state(tasks: &Vec<Task>, state: DefaultState) -> Vec<Task> {
    tasks
        .iter()
        .filter(|task| task.state == state)
        .cloned()
        .collect()
}
