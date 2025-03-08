use async_graphql::{EmptySubscription, Schema};
use crate::db::DbPool;

pub mod mutation;
pub mod query;

pub type AppSchema = Schema<query::Query, mutation::Mutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(query::Query::default(), mutation::Mutation::default(), EmptySubscription::default())
        .finish()
}

pub fn create_schema_with_db_pool(pool: DbPool) -> AppSchema {
    Schema::build(query::Query::default(), mutation::Mutation::default(), EmptySubscription::default())
        .data(pool)
        .finish()
}
