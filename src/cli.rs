// use crate::storage::FILE;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create the todo.toml state file
    Init,
    /// View the current state
    List, // {
    // #[arg(short, long)]
    // status: "DefaultState"
    // }
    /// Edit a task
    Edit {
        id: String,
        #[arg(short, long, required_unless_present = "title")]
        description: Option<String>,
        #[arg(short, long, required_unless_present = "description")]
        title: Option<String>,
    },
    /// Show a specific task
    Show { id: String },
    /// Add a new task in #todo
    Add {
        title: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Move a task in #doing
    Start { id: String },
    /// Change a task's status
    // Status {
    //     id: String,
    //     status: DefaultState
    // },
    /// Move an active task in #done
    Complete { id: String },
    /// Delete a task
    Delete { id: String },
    /// Reset the state file
    Clear,
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
