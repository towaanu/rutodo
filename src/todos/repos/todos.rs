use crate::todos::models::{NewTodo, Todo};
use tokio_postgres::{Client, Error};
use uuid::Uuid;

pub async fn find_todos_for_todo_list(
    pg_client: &Client,
    todo_list_id: &Uuid,
) -> Result<Vec<Todo>, Error> {
    let rows = pg_client
        .query(
            "
            SELECT 
                id, label, is_done, 
                todo_list_id, created_at, updated_at
            FROM todos
            WHERE todo_list_id = $1
            ",
            &[todo_list_id],
        )
        .await?;

    Ok(rows.iter().map(|row| row.into()).collect())
}

pub async fn find_todo_by_id(pg_client: &Client, id: i32) -> Result<Option<Todo>, Error> {
    pg_client
        .query_opt(
            "
                        SELECT 
                            id, label, is_done,
                            todo_list_id, created_at, updated_at
                        FROM todos
                        WHERE id = $1
                        ",
            &[&id],
        )
        .await
        .map(|opt| opt.map(|row| row.into()))
        .map_err(|err| err.into())
}

pub async fn create_todo(pg_client: &Client, new_todo: &NewTodo) -> Result<Todo, Error> {
    pg_client
        .query_one(
            "
                        INSERT INTO todos(label, todo_list_id)
                            VALUES ($1, $2)
                            RETURNING *
                        ",
            &[&new_todo.label, &new_todo.todo_list_id],
        )
        .await
        .map(|row| row.into())
        .map_err(|err| err.into())
}

pub async fn delete_todo(pg_client: &Client, id: i32) -> Result<u64, Error> {
    pg_client
        .execute("DELETE FROM todos WHERE id=$1", &[&id])
        .await
}

pub async fn toggle_todo(pg_client: &Client, id: i32) -> Result<Todo, Error> {
    let row = pg_client
        .query_one("SELECT is_done FROM todos WHERE id=$1", &[&id])
        .await?;

    let new_is_done = !row.get::<&str, bool>("is_done");
    pg_client
        .execute(
            "UPDATE todos SET is_done=$1 WHERE id=$2",
            &[&new_is_done, &id],
        )
        .await?;

    pg_client
        .query_one(
            "
                        SELECT 
                            id, label, is_done,
                            created_at, updated_at, todo_list_id
                        FROM todos
                        WHERE id=$1
                        ",
            &[&id],
        )
        .await
        .map(|row| row.into())
        .map_err(|err| err.into())
}
