use crate::graphql::{context::Context, errors::AppGraphQLResult};
use crate::todos::{models::NewTodoList, models::TodoList, todos};
use juniper::{graphql_object, EmptySubscription, ID};
use std::convert::TryFrom;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    #[graphql(description = "Current version of the API")]
    fn api_version() -> &str {
        "0.1"
    }

    #[graphql(description = "List all todo list")]
    async fn todo_lists(ctx: &Context) -> AppGraphQLResult<Vec<TodoList>> {
        let pg_client = ctx.get_pg_client().await;
        todos::find_todo_lists(&pg_client)
            .await
            .map_err(|err| err.into())
    }

    #[graphql(description = "Get todo list by id")]
    async fn todo_list(ctx: &Context, id: ID) -> AppGraphQLResult<Option<TodoList>> {
        let pg_client = ctx.get_pg_client().await;
        todos::find_todo_list_by_id(&pg_client, &id.parse().unwrap())
            .await
            .map_err(|err| err.into())
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    #[graphql(description = "Create a new todo list")]
    async fn create_todo_list(
        ctx: &Context,
        new_todo_list: NewTodoList,
    ) -> AppGraphQLResult<TodoList> {
        let pg_client = ctx.get_pg_client().await;
        todos::create_todo_list(&pg_client, &new_todo_list)
            .await
            .map_err(|err| err.into())
    }

    #[graphql(description = "Delete a todo list by id")]
    async fn delete_todo_list(ctx: &Context, id: ID) -> AppGraphQLResult<i32> {
        let pg_client = ctx.get_pg_client().await;
        todos::delete_todo_list(&pg_client, &id.parse().unwrap())
            .await
            .map(|res| i32::try_from(res).unwrap())
            .map_err(|err| err.into())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn new_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
