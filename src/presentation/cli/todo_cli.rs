use crate::application::use_cases::todo_service::TodoService;
use crate::domain::repositories::todo_repository::TodoRepository;
use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use uuid::Uuid;

pub struct TodoCli<R: TodoRepository> {
    service: TodoService<R>,
    theme: ColorfulTheme,
}

impl<R: TodoRepository> TodoCli<R> {
    pub fn new(service: TodoService<R>) -> Self {
        Self {
            service,
            theme: ColorfulTheme::default(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            self.display_todos().await?;

            let choices = &[
                "Adicionar tarefa",
                "Marcar/Desmarcar tarefa",
                "Remover tarefa",
                "Sair",
            ];

            let selection = Select::with_theme(&self.theme)
                .with_prompt("\nEscolha uma ação")
                .default(0)
                .items(&choices[..])
                .interact()?;

            match selection {
                0 => self.add_todo().await?,
                1 => self.toggle_todo().await?,
                2 => self.remove_todo().await?,
                3 => break,
                _ => unreachable!(),
            }
        }

        println!("{}", "👋 Até logo!".blue());
        Ok(())
    }

    async fn display_todos(&self) -> Result<()> {
        println!("\n{}:", "📝 Suas Tarefas".bold());
        let todos = self.service.list_todos().await?;

        if todos.is_empty() {
            println!("{}", "Nenhuma tarefa encontrada!".yellow());
            return Ok(());
        }

        for todo in todos {
            let status = if todo.completed {
                "✅".green()
            } else {
                "⭕".red()
            };

            let title = if todo.completed {
                todo.title.green()
            } else {
                todo.title.normal()
            };

            println!(
                "{} [{}] {} ({})",
                status,
                todo.id,
                title,
                todo.created_at.format("%d/%m/%Y %H:%M")
            );
        }

        Ok(())
    }

    async fn add_todo(&self) -> Result<()> {
        let title = Input::<String>::with_theme(&self.theme)
            .with_prompt("Digite o título da tarefa")
            .interact_text()?;

        self.service.create_todo(title).await?;
        println!("{}", "✨ Tarefa adicionada com sucesso!".green());
        Ok(())
    }

    async fn toggle_todo(&self) -> Result<()> {
        let todos = self.service.list_todos().await?;
        if todos.is_empty() {
            println!("{}", "Nenhuma tarefa para marcar/desmarcar!".yellow());
            return Ok(());
        }

        let id_str = Input::<String>::with_theme(&self.theme)
            .with_prompt("Digite o ID da tarefa")
            .interact_text()?;

        let id = Uuid::parse_str(&id_str)?;
        self.service.toggle_todo(&id).await?;
        println!("{}", "✨ Tarefa atualizada com sucesso!".green());
        Ok(())
    }

    async fn remove_todo(&self) -> Result<()> {
        let todos = self.service.list_todos().await?;
        if todos.is_empty() {
            println!("{}", "Nenhuma tarefa para remover!".yellow());
            return Ok(());
        }

        let id_str = Input::<String>::with_theme(&self.theme)
            .with_prompt("Digite o ID da tarefa para remover")
            .interact_text()?;

        let id = Uuid::parse_str(&id_str)?;
        self.service.delete_todo(&id).await?;
        println!("{}", "🗑️ Tarefa removida com sucesso!".red());
        Ok(())
    }
}
