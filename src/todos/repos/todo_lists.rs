use crate::todos::models::{NewTodoList, TodoList};
use tokio_postgres::{Client, Error};
use uuid::Uuid;

pub async fn find_todo_lists(pg_client: &Client) -> Result<Vec<TodoList>, Error> {
    let rows = pg_client
        .query(
            "
            SELECT id, label, created_at, updated_at
            FROM todo_lists
            ",
            &[],
        )
        .await?;

    Ok(rows.iter().map(|row| row.into()).collect())
}

pub async fn find_todo_list_by_id(
    pg_client: &Client,
    id: &Uuid,
) -> Result<Option<TodoList>, Error> {
    pg_client
        .query_opt(
            "
                SELECT id, label, created_at, updated_at
                FROM todo_lists
                WHERE id=$1
            ",
            &[id],
        )
        .await
        .map(|opt| opt.map(|row| row.into()))
        .map_err(|err| err.into())
}

pub async fn create_todo_list(
    pg_client: &Client,
    new_todo_list: &NewTodoList,
) -> Result<TodoList, Error> {
    pg_client
        .query_one(
            "INSERT INTO todo_lists(label)
                   VALUES ($1)
                   RETURNING *",
            &[&new_todo_list.label],
        )
        .await
        .map(|row| row.into())
        .map_err(|err| err.into())
}

pub async fn delete_todo_list(pg_client: &Client, id: &Uuid) -> Result<u64, Error> {
    pg_client
        .execute("DELETE FROM todo_lists WHERE id=$1", &[id])
        .await
}
