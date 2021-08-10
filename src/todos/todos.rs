use crate::errors::ServiceResult;
use crate::todos::{models::NewTodo, models::NewTodoList, models::Todo, models::TodoList, repos};
use tokio_postgres::Client;
use uuid::Uuid;

pub async fn find_todo_lists(pg_client: &Client) -> ServiceResult<Vec<TodoList>> {
    repos::todo_lists::find_todo_lists(&pg_client)
        .await
        .map_err(|err| err.into())
}

pub async fn find_todo_list_by_id(
    pg_client: &Client,
    id: &Uuid,
) -> ServiceResult<Option<TodoList>> {
    repos::todo_lists::find_todo_list_by_id(&pg_client, id)
        .await
        .map_err(|err| err.into())
}

pub async fn create_todo_list(
    pg_client: &Client,
    new_todo_list: &NewTodoList,
) -> ServiceResult<TodoList> {
    repos::todo_lists::create_todo_list(pg_client, new_todo_list)
        .await
        .map_err(|err| err.into())
}

pub async fn delete_todo_list(pg_client: &Client, id: &Uuid) -> ServiceResult<u64> {
    repos::todo_lists::delete_todo_list(pg_client, id)
        .await
        .map_err(|err| err.into())
}

pub async fn find_todos_for_todo_list(
    pg_client: &Client,
    todo_list_id: &Uuid,
) -> ServiceResult<Vec<Todo>> {
    repos::todos::find_todos_for_todo_list(pg_client, todo_list_id)
        .await
        .map_err(|err| err.into())
}

pub async fn find_todo_by_id(pg_client: &Client, id: i32) -> ServiceResult<Option<Todo>> {
    repos::todos::find_todo_by_id(pg_client, id)
        .await
        .map_err(|err| err.into())
}

pub async fn create_todo(pg_client: &Client, new_todo: &NewTodo) -> ServiceResult<Todo> {
    repos::todos::create_todo(pg_client, new_todo)
        .await
        .map_err(|err| err.into())
}

pub async fn toggle_todo(pg_client: &Client, id: i32) -> ServiceResult<Todo> {
    repos::todos::toggle_todo(pg_client, id)
        .await
        .map_err(|err| err.into())
}

pub async fn delete_todo(pg_client: &Client, id: i32) -> ServiceResult<u64> {
    repos::todos::delete_todo(pg_client, id)
        .await
        .map_err(|err| err.into())
}
