use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    vec,
};
use thiserror::Error;

#[derive(Subcommand, Debug)]
enum Command {
    /// View the current state
    View {
        #[arg(short, long)]
        compact: bool,
    },
    /// Add a new task in #todo
    Add { title: String },
    /// Move a task in #doing
    Start,
    /// Move an active task in #done
    Complete,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Command,

    /// State file
    #[arg(default_value_t = String::from("todo.toml"))]
    file: String,
}

#[derive(Error, Debug)]
enum StateError {
    #[error("Failed to read state file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to serialize state: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("Failed to parse state file: {0}")]
    DeserializeError(#[from] toml::de::Error),
}

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
    description: String,
    estimated_duration: String,
    time_spent: String,
    tags: Vec<String>,
}

impl Task {
    fn new(title: String) -> Self {
        Self {
            title,
            description: String::new(),
            estimated_duration: String::new(),
            time_spent: String::new(),
            tags: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
struct State {
    todo: Vec<Task>,
    done: Vec<Task>,
    doing: Vec<Task>,
}

impl State {
    fn new() -> Self {
        Self {
            todo: vec![],
            done: vec![],
            doing: vec![],
        }
    }

    fn write_state(&self, file_path: &String) -> Result<(), StateError> {
        let res = toml::to_string_pretty(self)?;
        let mut file = File::create(file_path)?;

        file.write_all(res.as_bytes())?;

        Ok(())
    }

    fn load_state(&mut self, file_path: &String) -> Result<(), StateError> {
        let file_content = fs::read_to_string(file_path)?;
        let parsed = toml::from_str::<State>(&file_content)?;

        self.doing = parsed.doing;
        self.done = parsed.done;
        self.todo = parsed.todo;

        Ok(())
    }
}

fn main() -> Result<(), StateError> {
    let mut state = State::new();

    let args = Args::parse();
    let _ = state.load_state(&args.file);

    match args.command {
        Command::View { compact } => view_state(&args.file, compact)?,
        Command::Add { title } => add_task(state, &args.file, title)?,
        Command::Start => todo!(),
        Command::Complete => todo!(),
    }

    Ok(())
}

fn view_state(state_file_path: &str, compact: bool) -> Result<(), StateError> {
    let mut file_content = fs::read_to_string(state_file_path)?;
    if compact {
        file_content = file_content
            .lines()
            .filter(|line| line.contains("title ="))
            .map(|line| {
                line.replace("title =", "")
                    .trim()
                    .trim_matches('"')
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    println!("{}", file_content);
    Ok(())
}

fn add_task(
    mut state: State,
    state_file_path: &String,
    task_title: String,
) -> Result<(), StateError> {
    state.todo.push(Task::new(task_title));

    state.write_state(state_file_path)?;

    Ok(())
}
