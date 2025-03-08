use async_graphql::{Context, Object, Result};
use diesel::prelude::*;

use crate::db::{models::ChatMessage, DbPool};
use crate::db::schema::chat_messages;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    /// Returns a greeting message
    async fn hello(&self) -> Result<&str> {
        Ok("hello")
    }

    /// Get all chat messages
    async fn chat_messages(&self, ctx: &Context<'_>) -> Result<Vec<ChatMessage>> {
        let pool = ctx.data::<DbPool>()?;
        let mut conn = pool.get()?;

        let messages = chat_messages::table
            .order_by(chat_messages::timestamp.desc())
            .load::<ChatMessage>(&mut conn)?;

        Ok(messages)
    }

    /// Get a single chat message by ID
    async fn chat_message(&self, ctx: &Context<'_>, id: i32) -> Result<Option<ChatMessage>> {
        let pool = ctx.data::<DbPool>()?;
        let mut conn = pool.get()?;

        let message = chat_messages::table
            .find(id)
            .first::<ChatMessage>(&mut conn)
            .optional()?;

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use chrono::Utc;
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::sqlite::SqliteConnection;
    use crate::db::models::NewChatMessage;
    use crate::schema::mutation::Mutation;

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

    #[tokio::test]
    async fn test_chat_messages_query() {
        // Set up an in-memory SQLite database for testing
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create test db pool");

        // Create the table in the in-memory database
        let mut conn = pool.get().unwrap();
        diesel::sql_query(
            "CREATE TABLE chat_messages (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                sender TEXT NOT NULL,
                timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&mut conn).unwrap();

        // Insert test data
        let test_message = NewChatMessage {
            content: "Test message".to_string(),
            sender: "Test User".to_string(),
            timestamp: Utc::now().naive_utc(),
        };

        diesel::insert_into(chat_messages::table)
            .values(&test_message)
            .execute(&mut conn)
            .unwrap();

        // Create a schema with the pool as context data
        let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription::default())
            .data(pool.clone())
            .finish();

        // Test chatMessages query
        let query = "{ chatMessages { id content sender } }";
        let res = schema.execute(query).await;
        assert!(res.is_ok());

        let json_str = serde_json::to_string(&res.data).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        
        assert!(json_value["chatMessages"].is_array());
        assert_eq!(json_value["chatMessages"][0]["content"], "Test message");
        assert_eq!(json_value["chatMessages"][0]["sender"], "Test User");

        // Test chatMessage query
        let query = "{ chatMessage(id: 1) { id content sender } }";
        let res = schema.execute(query).await;
        assert!(res.is_ok());

        let json_str = serde_json::to_string(&res.data).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(json_value["chatMessage"]["content"], "Test message");
        assert_eq!(json_value["chatMessage"]["sender"], "Test User");
    }
}
