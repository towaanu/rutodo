use crate::graphql::context::Context;
use crate::graphql::errors::AppGraphQLResult;
use crate::todos::models::{Todo, TodoList};
use crate::todos::todos::find_todo_list_by_id;
use chrono::{DateTime, Utc};
use juniper::{graphql_object, ID};

#[graphql_object(context = Context)]
impl Todo {
    fn id(&self) -> ID {
        self.id.to_string().into()
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn is_done(&self) -> bool {
        self.is_done
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    async fn todo_list(&self, ctx: &Context) -> AppGraphQLResult<TodoList> {
        let pg_client = ctx.get_pg_client().await;
        find_todo_list_by_id(&pg_client, &self.todo_list_id)
            .await
            .map(|opt| opt.unwrap())
            .map_err(|err| err.into())
    }
}
