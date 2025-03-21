use async_graphql::{Context, Object, Result};
use base64;
use chrono::NaiveDateTime;
use diesel::{prelude::*, QueryableByName};

use crate::db::{models::ChatMessage, DbPool};
use crate::db::schema::chat_messages;

#[derive(async_graphql::SimpleObject)]
struct PageInfo {
    has_next_page: bool,
    end_cursor: Option<String>,
}

#[derive(async_graphql::SimpleObject)]
struct ChatMessageEdge {
    node: ChatMessage,
    cursor: String,
}

#[derive(async_graphql::SimpleObject)]
struct ChatMessageConnection {
    edges: Vec<ChatMessageEdge>,
    page_info: PageInfo,
}

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    /// Returns a greeting message
    async fn hello(&self) -> Result<&str> {
        Ok("hello")
    }

    /// Get paginated chat messages
    async fn chat_messages(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>
    ) -> Result<ChatMessageConnection> {
        let pool = ctx.data::<DbPool>()?;
        let mut conn = pool.get()?;

        let limit = first.unwrap_or(20).min(100) as i64;
        let mut query = chat_messages::table
            .order_by(chat_messages::timestamp.desc())
            .into_boxed();

        if let Some(cursor) = after {
            let decoded_cursor = base64::decode(cursor)?;
            let cursor_str = String::from_utf8(decoded_cursor)?;
            let cursor_timestamp = cursor_str.parse::<NaiveDateTime>()?;
            query = query.filter(chat_messages::timestamp.lt(cursor_timestamp));
        }

        let messages = query
            .limit(limit)
            .load::<ChatMessage>(&mut conn)?;

        let has_next_page = messages.len() as i64 == limit;
        let end_cursor = messages.last()
            .map(|msg| base64::encode(msg.timestamp.to_string()));

        Ok(ChatMessageConnection {
            edges: messages.into_iter().map(|msg| {
                let timestamp = msg.timestamp.to_string();
                ChatMessageEdge {
                    node: msg,
                    cursor: base64::encode(timestamp),
                }
            }).collect(),
            page_info: PageInfo {
                has_next_page,
                end_cursor,
            },
        })
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
        // Use a file-based database for testing to ensure persistence across connections
        let manager = ConnectionManager::<SqliteConnection>::new("file:test_db?mode=memory&cache=shared");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create test db pool");

        // Run migrations to create the tables
        let mut conn = pool.get().unwrap();
        use diesel_migrations::MigrationHarness;
        use crate::db::migrations::MIGRATIONS;
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
        
        // Verify the table exists
        #[derive(QueryableByName)]
        struct TableName {
            #[diesel(sql_type = diesel::sql_types::Text)]
            name: String,
        }
        
        let tables = diesel::sql_query("SELECT name FROM sqlite_master WHERE type='table' AND name='chat_messages'")
            .load::<TableName>(&mut conn)
            .unwrap();
        assert!(!tables.is_empty(), "chat_messages table was not created by migrations");

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
        let query = "{ chatMessages { edges { node { id content sender } } pageInfo { hasNextPage endCursor } } }";
        let res = schema.execute(query).await;
        println!("res: {:?}", res);
        assert!(res.is_ok());

        let json_str = serde_json::to_string(&res.data).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        
        assert!(json_value["chatMessages"]["edges"].is_array());
        assert_eq!(json_value["chatMessages"]["edges"][0]["node"]["content"], "Test message");
        assert_eq!(json_value["chatMessages"]["edges"][0]["node"]["sender"], "Test User");

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
