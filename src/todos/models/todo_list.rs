use chrono::{DateTime, Utc};
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub id: Uuid,
    pub label: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
#[graphql(name = "TodoListInput")]
pub struct NewTodoList {
    pub label: String,
}

impl From<Row> for TodoList {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            label: row.get("label"),

            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl From<&Row> for TodoList {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            label: row.get("label"),

            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
