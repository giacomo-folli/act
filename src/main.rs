use crate::{
    cli::{Args, Command},
    models::{StateError, Task},
};
use clap::Parser;

mod cli;
mod commands;
mod models;
mod storage;

fn main() -> anyhow::Result<()> {
    // let mut state = State { tasks: vec![] };

    let args = Args::parse();

    match args.command {
        Command::Init => storage::init()?,
        Command::List => {
            let tasks = storage::load()?;
            commands::list_tasks(&tasks)
        }
        Command::Add { title, description } => add_task(title, &description)?,
        Command::Edit {
            id,
            title,
            description,
        } => edit_task(id, &title, &description)?,
        Command::Start { id } => start_task(id)?,
        Command::Show { id } => show_task(id)?,
        Command::Delete { id } => delete_task(id)?,
        Command::Complete { id } => complete_task(id)?,
        Command::Clear => clear_state()?,
    }

    Ok(())
}

fn show_task(task_id: String) -> anyhow::Result<()> {
    let tasks = storage::load()?;
    let mut file_content = String::new();

    for task in tasks.iter() {
        if task.id == task_id {
            file_content = toml::to_string(&task)?;
            break;
        }
    }

    if file_content.is_empty() {
        file_content = "No task found with that id.".to_string();
    }

    println!("{}", file_content);
    Ok(())
}

fn start_task(task_id: String) -> anyhow::Result<()> {
    let mut tasks = storage::load()?;

    for task in tasks.iter_mut() {
        if task.id == task_id {
            task.state = models::DefaultState::Doing;
            break;
        }
    }

    let _ = storage::save(&tasks);

    Ok(())
}

fn complete_task(task_id: String) -> anyhow::Result<()> {
    let mut tasks = storage::load()?;

    for task in tasks.iter_mut() {
        if task.id == task_id {
            task.state = models::DefaultState::Done;
            break;
        }
    }

    let _ = storage::save(&tasks);

    Ok(())
}

fn add_task(task_title: String, task_description: &Option<String>) -> Result<(), StateError> {
    let mut tasks = storage::load()?;
    let mut new_task = Task::new(task_title);

    if let Some(desc) = task_description {
        new_task.description = Some(desc.clone());
    }

    tasks.push(new_task);

    let _ = storage::save(&tasks);

    Ok(())
}

fn edit_task(
    id: String,
    task_title: &Option<String>,
    task_description: &Option<String>,
) -> Result<(), StateError> {
    let mut tasks = storage::load()?;

    for task in tasks.iter_mut() {
        if task.id == id {
            if let Some(desc) = task_description {
                task.description = Some(desc.clone());
                task.update_time();
            }

            if let Some(title) = task_title {
                task.title = title.clone();
                task.update_time();
            }
        }
    }

    let _ = storage::save(&tasks);

    Ok(())
}

fn delete_task(task_id: String) -> anyhow::Result<()> {
    let mut tasks = storage::load()?;

    tasks = tasks
        .iter()
        .filter(|task| task.id != task_id)
        .cloned()
        .collect();

    let _ = storage::save(&tasks);

    Ok(())
}

fn clear_state() -> anyhow::Result<()> {
    let _ = storage::save(&vec![]);

    Ok(())
}
