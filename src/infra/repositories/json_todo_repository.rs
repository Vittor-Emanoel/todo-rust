use crate::domain::{entities::todo::Todo, repositories::todo_repository::TodoRepository};
use crate::infra::persistence::json_storage::JsonStorage;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

pub struct JsonTodoRepository {
    storage: Mutex<JsonStorage>,
}

#[derive(Serialize, Deserialize, Default)]
struct TodoStorage {
    todos: Vec<Todo>,
}

impl JsonTodoRepository {
    pub fn new(storage: JsonStorage) -> Self {
        Self {
            storage: Mutex::new(storage),
        }
    }
}

#[async_trait]
impl TodoRepository for JsonTodoRepository {
    async fn save(&self, todo: &Todo) -> Result<()> {
        let storage = self.storage.lock().unwrap();
        let mut data = storage.read::<TodoStorage>()?.unwrap_or_default();
        data.todos.push(todo.clone());
        storage.write(&data)?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Todo>> {
        let storage = self.storage.lock().unwrap();
        let data = storage.read::<TodoStorage>()?.unwrap_or_default();
        Ok(data.todos)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Todo>> {
        let storage = self.storage.lock().unwrap();
        let data = storage.read::<TodoStorage>()?.unwrap_or_default();
        Ok(data.todos.into_iter().find(|t| &t.id == id))
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        let storage = self.storage.lock().unwrap();
        let mut data = storage.read::<TodoStorage>()?.unwrap_or_default();
        data.todos.retain(|t| &t.id != id);
        storage.write(&data)?;
        Ok(())
    }

    async fn update(&self, todo: &Todo) -> Result<()> {
        let storage = self.storage.lock().unwrap();
        let mut data = storage.read::<TodoStorage>()?.unwrap_or_default();
        if let Some(index) = data.todos.iter().position(|t| t.id == todo.id) {
            data.todos[index] = todo.clone();
            storage.write(&data)?;
        }
        Ok(())
    }
}
