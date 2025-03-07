use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub mod query;

pub type AppSchema = Schema<query::Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(query::Query::default(), EmptyMutation::default(), EmptySubscription::default())
        .finish()
}
