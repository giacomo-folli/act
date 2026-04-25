use anyhow::Result;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::models::{State, StateError, Task};

pub const FILE: &str = "tasks.toml";

pub fn init() -> anyhow::Result<()> {
    let _ = File::create(FILE)?;

    Ok(())
}

pub fn load() -> Result<Vec<Task>, StateError> {
    if !std::path::Path::new(FILE).exists() {
        let _ = init();
    }

    let raw = fs::read_to_string(FILE)?;
    let parsed: State = toml::from_str(&raw)?;
    Ok(parsed.tasks)
}

pub fn save(tasks: &[Task]) -> Result<(), StateError> {
    let state = State {
        tasks: tasks.to_owned(),
    };

    let res = toml::to_string(&state)?;

    let mut file = File::create(FILE)?;

    file.write_all(res.as_bytes())?;

    Ok(())
}
