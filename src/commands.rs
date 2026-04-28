use chrono::DateTime;
use colored::Colorize;
use comfy_table::{
    Attribute, Cell, Color, ColumnConstraint, ContentArrangement, Table, Width, presets::UTF8_HORIZONTAL_ONLY,
};

use crate::models::{DefaultState, Task};

pub fn list_tasks(tasks: &[Task]) {
    let mut table = Table::new();

    table.load_preset(UTF8_HORIZONTAL_ONLY).set_content_arrangement(ContentArrangement::DynamicFullWidth).set_header(
        vec![
            Cell::new("ID").add_attribute(Attribute::Dim),
            Cell::new("TITLE").add_attribute(Attribute::Dim),
            Cell::new("STATUS").add_attribute(Attribute::Dim),
            Cell::new("UPDATED").add_attribute(Attribute::Dim),
        ],
    );

    table.column_mut(1).unwrap().set_constraint(ColumnConstraint::LowerBoundary(Width::Fixed(30)));

    for task in tasks {
        table.add_row(vec![
            Cell::new(&task.id[..8]).fg(Color::DarkGrey),
            Cell::new(format_title(task)),
            format_status(&task.state),
            Cell::new(format_relative_time(&task.updated_at)).fg(Color::DarkGrey),
        ]);
    }

    println!("{table}");
    println!("{}", format_footer(tasks));
}

fn format_relative_time(timestamp_str: &str) -> String {
    if let Ok(dt) = DateTime::parse_from_rfc2822(timestamp_str) {
        let secs = chrono::Local::now().signed_duration_since(dt).num_seconds();
        match secs {
            s if s < 3600 => format!("{}m ago", s / 60),
            s if s < 86400 => format!("{}h ago", s / 3600),
            s if s < 86400 * 2 => "yesterday".into(),
            s => format!("{}d ago", s / 86400),
        }
    } else {
        "unknown".to_string()
    }
}

fn format_footer(tasks: &[Task]) -> String {
    let done = tasks.iter().filter(|t| t.state == DefaultState::Done).count();
    let doing = tasks.iter().filter(|t| t.state == DefaultState::Doing).count();
    let todo = tasks.iter().filter(|t| t.state == DefaultState::Todo).count();

    format!(
        "{} tasks  ·  {} done  ·  {} doing  ·  {} todo",
        tasks.len().to_string().dimmed(),
        done.to_string().bold(),
        doing.to_string().bold(),
        todo.to_string().bold(),
    )
}

fn format_status(status: &DefaultState) -> Cell {
    match status {
        DefaultState::Todo => Cell::new(" todo  ").fg(Color::Grey).bg(Color::AnsiValue(237)),
        DefaultState::Doing => Cell::new(" doing ").fg(Color::Blue).bg(Color::AnsiValue(17)),
        DefaultState::Done => Cell::new(" done  ").fg(Color::Green).bg(Color::AnsiValue(22)),
    }
}

fn format_title(task: &Task) -> String {
    match &task.description {
        Some(desc) => format!("{}\n{}", task.title, desc.dimmed()),
        None => task.title.clone(),
    }
}
