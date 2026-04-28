// use crate::storage::FILE;

use clap::{Parser, Subcommand};

use crate::models::DefaultState;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create the todo.toml state file
    Init,
    /// View the current state
    List {
        #[arg(short, long)]
        status: Option<DefaultState>,
    },
    /// Edit a task
    Edit {
        id:          String,
        #[arg(short, long, required_unless_present = "title")]
        description: Option<String>,
        #[arg(short, long, required_unless_present = "description")]
        title:       Option<String>,
    },
    /// Show a specific task
    Show { id: String },
    /// Add a new task in #todo
    Add {
        title:       Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Change a task's status
    Status { id: String, status: DefaultState },
    /// Delete a task
    Delete { id: String },
}

/// Simple task managment cli tool
#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    pub command: Command,
    // State file
    // #[arg(default_value_t = String::from(FILE))]
    // pub file: String,
}
