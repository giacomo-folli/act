use crate::state::{State, StateError};
use crate::task::Task;

mod state;
mod task;

use clap::{Parser, Subcommand};
use std::fs;

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
    Start { id: String },
    /// Move an active task in #done
    Complete,
}

/// Simple task managment cli tool
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

fn main() -> Result<(), StateError> {
    let mut state = State::new();

    let args = Args::parse();
    match state.load_state(&args.file) {
        Ok(()) => {}
        Err(err) => return Err(err),
    }

    match args.command {
        Command::View { compact } => view_state(&args.file, compact)?,
        Command::Add { title } => add_task(&mut state, &args.file, title)?,
        Command::Start { id } => start_task(&mut state, &args.file, id)?,
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

    if file_content.is_empty() {
        file_content = "The state is empty! Try to add some tasks.".to_string();
    }

    println!("{}", file_content);
    Ok(())
}

fn start_task(state: &mut State, state_file_path: &str, task_id: String) -> Result<(), StateError> {
    if state.todo.is_empty() {
        println!("No tasks in todo! Try to add one.");
        return Ok(());
    }

    for task in state.todo.iter_mut() {
        if task.id == task_id {
            task.update_state(task::DefaultState::Doing);
            break;
        }
    }

    state.write_state(state_file_path)?;

    Ok(())
}

fn add_task(
    state: &mut State,
    state_file_path: &str,
    task_title: String,
) -> Result<(), StateError> {
    state.todo.push(Task::new(task_title));

    state.write_state(state_file_path)?;

    Ok(())
}
