use async_graphql::{Object, Result};

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    /// Returns a greeting message
    async fn hello(&self) -> Result<&str> {
        Ok("hello")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};

    #[tokio::test]
    async fn test_hello_query() {
        // Create a schema just for testing
        let schema = Schema::build(Query::default(), EmptyMutation::default(), EmptySubscription::default())
            .finish();
        
        // Execute the query
        let query = "{ hello }";
        let res = schema.execute(query).await;
        
        // Convert to JSON and verify
        let json_str = serde_json::to_string(&res.data).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(json_value["hello"], "hello");
    }
}
