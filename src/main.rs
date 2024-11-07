use std::path::PathBuf;
use todorust::{
    application::use_cases::todo_service::TodoService,
    infra::{
        persistence::json_storage::JsonStorage,
        repositories::json_todo_repository::JsonTodoRepository,
    },
    presentation::cli::todo_cli::TodoCli,
};

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    let storage = JsonStorage::new(PathBuf::from("todos.json"));
    let repository = JsonTodoRepository::new(storage);
    let service = TodoService::new(repository);
    let cli = TodoCli::new(service);

    rt.block_on(async { cli.run().await })
}
