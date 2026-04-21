use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum DefaultState {
    Todo,
    Doing,
    Done,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub state: DefaultState,
    pub description: String,
    pub estimated_duration: String,
    pub time_spent: String,
    pub created_at: String,
    pub tags: Vec<String>,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: nanoid!(8),
            state: DefaultState::Todo,
            description: String::new(),
            estimated_duration: String::new(),
            time_spent: String::new(),
            created_at: chrono::Local::now().to_string(),
            tags: vec![],
            title,
        }
    }

    pub fn update_state(&mut self, new_state: DefaultState) {
        self.state = new_state;
    }
}
