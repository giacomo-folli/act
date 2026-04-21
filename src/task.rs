use nanoid::nanoid;
use serde::{Deserialize, Serialize};

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
    pub created_at: String,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: nanoid!(8),
            state: DefaultState::Todo,
            description: String::new(),
            created_at: chrono::Local::now().to_string(),
            title,
        }
    }

    pub fn update_state(&mut self, new_state: DefaultState) {
        self.state = new_state;
    }
}
