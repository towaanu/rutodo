use chrono::{DateTime, Utc};
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub label: String,
    #[serde(rename = "isDone")]
    pub is_done: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    #[serde(rename = "todoListId")]
    pub todo_list_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
#[graphql(name = "TodoInput")]
pub struct NewTodo {
    pub label: String,
    #[serde(rename = "todoListId")]
    pub todo_list_id: Uuid,
}

impl From<Row> for Todo {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            label: row.get("label"),
            is_done: row.get("is_done"),

            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),

            todo_list_id: row.get("todo_list_id"),
        }
    }
}

impl From<&Row> for Todo {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            label: row.get("label"),
            is_done: row.get("is_done"),

            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),

            todo_list_id: row.get("todo_list_id"),
        }
    }
}
