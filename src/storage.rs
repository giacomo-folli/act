use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Context;
use dirs::config_dir;

use crate::models::{Storage, Task};

pub const FILE: &str = "tasks.toml";

pub fn get_config() -> anyhow::Result<PathBuf> {
    let mut config_path = config_dir().context("Could not find config directory")?;

    config_path.push("grind");

    if !config_path.exists() {
        fs::create_dir_all(&config_path).context("Failed to create config directory")?;
    }

    Ok(config_path.join(FILE))
}

pub fn init() -> anyhow::Result<()> {
    let file_path = get_config()?;

    if !file_path.exists() {
        File::create(&file_path).context("Failed to create tasks file")?;
    }

    Ok(())
}

pub fn load() -> anyhow::Result<Vec<Task>> {
    let file_path = get_config()?;

    let raw = fs::read_to_string(file_path).context("Failed to read tasks file")?;
    let parsed: Storage = toml::from_str(&raw).context("Failed to parse tasks file")?;

    Ok(parsed.tasks)
}

pub fn save(tasks: &[Task]) -> anyhow::Result<()> {
    let file_path = get_config()?;
    let storage = Storage { tasks: tasks.to_owned() };

    let res = toml::to_string(&storage).context("Failed to serialize tasks")?;
    let mut file = File::create(file_path).context("Failed to create tasks file")?;

    file.write_all(res.as_bytes()).context("Failed to write tasks file")?;

    Ok(())
}
