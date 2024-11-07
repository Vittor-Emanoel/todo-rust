use crate::domain::entities::todo::Todo;
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn save(&self, todo: &Todo) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<Todo>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Todo>>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
    async fn update(&self, todo: &Todo) -> Result<()>;
}
