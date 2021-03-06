use crate::graphql::context::Context;
use crate::graphql::errors::AppGraphQLResult;
use crate::todos::models::{Todo, TodoList};
use crate::todos::todos::find_todos_for_todo_list;
use chrono::{DateTime, Utc};
use juniper::{graphql_object, ID};

#[graphql_object(context = Context)]
impl TodoList {
    fn id(&self) -> ID {
        self.id.to_string().into()
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    async fn todos(&self, ctx: &Context) -> AppGraphQLResult<Vec<Todo>> {
        let pg_client = ctx.get_pg_client().await;
        find_todos_for_todo_list(&pg_client, &self.id)
            .await
            .map_err(|err| err.into())
    }
}
