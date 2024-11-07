use crate::domain::{entities::todo::Todo, repositories::todo_repository::TodoRepository};
use anyhow::Result;
use uuid::Uuid;

pub struct TodoService<R: TodoRepository> {
    repository: R,
}

impl<R: TodoRepository> TodoService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_todo(&self, title: String) -> Result<Todo> {
        let todo = Todo::new(title);
        self.repository.save(&todo).await?;
        Ok(todo)
    }

    pub async fn toggle_todo(&self, id: &Uuid) -> Result<()> {
        if let Some(mut todo) = self.repository.find_by_id(id).await? {
            todo.toggle_completion();
            self.repository.update(&todo).await?;
        }
        Ok(())
    }

    pub async fn delete_todo(&self, id: &Uuid) -> Result<()> {
        self.repository.delete(id).await
    }

    pub async fn list_todos(&self) -> Result<Vec<Todo>> {
        self.repository.find_all().await
    }
}
