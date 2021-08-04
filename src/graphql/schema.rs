use crate::graphql::context::Context;
use juniper::{graphql_object, EmptyMutation, EmptySubscription};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    #[graphql(description = "Current version of the API")]
    fn api_version() -> &str {
        "0.1"
    }
}

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn new_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
