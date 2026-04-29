use clap::ValueEnum;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefaultStatus {
    Todo,
    Doing,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    #[serde(default)]
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id:          String,
    pub title:       String,
    pub description: Option<String>,
    pub status:      DefaultStatus,
    pub created_at:  String,
    pub updated_at:  String,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: nanoid!(8),
            status: DefaultStatus::Todo,
            created_at: chrono::Local::now().to_rfc2822(),
            updated_at: chrono::Local::now().to_rfc2822(),
            description: None,
            title,
        }
    }

    pub fn update_time(&mut self) {
        self.updated_at = chrono::Local::now().to_rfc2822();
    }
}
