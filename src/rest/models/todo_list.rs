use crate::todos::models::{Todo, TodoList as LazyTodoList};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub id: Uuid,
    pub label: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new(todo_list: LazyTodoList, todos: Vec<Todo>) -> Self {
        Self {
            id: todo_list.id,
            label: todo_list.label,
            created_at: todo_list.created_at,
            updated_at: todo_list.updated_at,
            todos,
        }
    }
}
