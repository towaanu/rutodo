use crate::graphql::context::Context;
use crate::todos::models::TodoList;
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
}
