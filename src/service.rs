use std::sync::Arc;

use crate::{
    errors::TaskError,
    models::{DefaultStatus, Task},
    storage::{FileStorage, StorageBackend},
};

pub struct TaskService {
    storage: Arc<dyn StorageBackend>,
}

impl TaskService {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(FileStorage),
        }
    }

    #[allow(dead_code)]
    pub fn with_storage(storage: Arc<dyn StorageBackend>) -> Self {
        Self { storage }
    }

    pub fn search_alias(&self, task_alias: String) -> Option<String> {
        let tasks = self.storage.load().expect("Failed to read tasks file");

        if let Some(found) = tasks.iter().find(|t| t.id == task_alias) {
            Some(found.id.clone())
        } else if let Some(ff) = tasks.iter().find(|t| t.title == task_alias) {
            Some(ff.id.clone())
        } else if let Some(ff) =
            tasks.iter().find(|t| t.id.starts_with(&task_alias))
        {
            Some(ff.id.clone())
        } else {
            None
        }
    }

    pub fn list_tasks(
        &self,
        filter: Option<DefaultStatus>,
    ) -> anyhow::Result<Vec<Task>> {
        let tasks = self.storage.load()?;
        Ok(match filter {
            Some(status) => tasks
                .iter()
                .filter(|t| t.status == status)
                .cloned()
                .collect(),
            None => tasks,
        })
    }

    pub fn add_task(
        &self,
        title: Option<String>,
        description: Option<String>,
    ) -> anyhow::Result<()> {
        let mut tasks = self.storage.load()?;
        let title = title.unwrap_or_else(|| "New task".to_string());
        let mut new_task = Task::new(title);
        new_task.description = description;
        tasks.push(new_task);
        self.storage.save(&tasks)
    }

    pub fn edit_task(
        &self,
        task_alias: &str,
        title: Option<String>,
        description: Option<String>,
    ) -> anyhow::Result<()> {
        let mut tasks = self.storage.load()?;
        let task_id = match self.search_alias(task_alias.to_string()) {
            Some(s) => s,
            None => {
                return Err(
                    TaskError::TaskNotFound(task_alias.to_string()).into()
                );
            },
        };

        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            if let Some(desc) = description {
                task.description = Some(desc);
            }
            if let Some(new_title) = title {
                task.title = new_title;
            }
            task.update_time();
            self.storage.save(&tasks)
        } else {
            Err(TaskError::TaskNotFound(task_id.to_string()).into())
        }
    }

    pub fn update_status(
        &self,
        task_alias: &str,
        status: DefaultStatus,
    ) -> anyhow::Result<()> {
        let mut tasks = self.storage.load()?;
        let task_id = match self.search_alias(task_alias.to_string()) {
            Some(s) => s,
            None => {
                return Err(
                    TaskError::TaskNotFound(task_alias.to_string()).into()
                );
            },
        };

        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = status;
            task.update_time();
            self.storage.save(&tasks)
        } else {
            Err(TaskError::TaskNotFound(task_id.to_string()).into())
        }
    }

    pub fn delete_task(&self, task_alias: &str) -> anyhow::Result<()> {
        let mut tasks = self.storage.load()?;
        let task_id = match self.search_alias(task_alias.to_string()) {
            Some(s) => s,
            None => {
                return Err(
                    TaskError::TaskNotFound(task_alias.to_string()).into()
                );
            },
        };

        if tasks.iter().any(|t| t.id == task_id) {
            tasks.retain(|t| t.id != task_id);
            self.storage.save(&tasks)
        } else {
            Err(TaskError::TaskNotFound(task_id.to_string()).into())
        }
    }
}

impl Default for TaskService {
    fn default() -> Self {
        Self::new()
    }
}
