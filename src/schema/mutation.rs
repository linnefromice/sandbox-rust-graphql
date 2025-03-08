use async_graphql::{Context, Object, Result};
use chrono::Utc;
use diesel::{prelude::*, QueryableByName};

use crate::db::{models::{ChatMessage, NewChatMessage}, DbPool};
use crate::db::schema::chat_messages;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a new chat message
    async fn create_chat_message(
        &self,
        ctx: &Context<'_>,
        content: String,
        sender: String,
    ) -> Result<ChatMessage> {
        let pool = ctx.data::<DbPool>()?;
        let mut conn = pool.get()?;

        let new_message = NewChatMessage {
            content,
            sender,
            timestamp: Utc::now().naive_utc(),
        };

        // For SQLite, we need to use execute and then get the last inserted row
        diesel::insert_into(chat_messages::table)
            .values(&new_message)
            .execute(&mut conn)?;

        let message = chat_messages::table
            .order(chat_messages::id.desc())
            .first(&mut conn)?;

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{EmptySubscription, Schema};
    use crate::schema::query::Query;
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::sqlite::SqliteConnection;

    #[tokio::test]
    async fn test_create_chat_message() {
        // Set up an in-memory SQLite database for testing
        // Use a file-based database for testing to ensure persistence across connections
        let manager = ConnectionManager::<SqliteConnection>::new("file:test_mutation_db?mode=memory&cache=shared");
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

        // Create a schema with the pool as context data
        let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription::default())
            .data(pool.clone())
            .finish();

        // Define test message content
        let content = "Hello, world!";
        let sender = "Test User";

        // Execute the mutation
        let query = format!(r#"
            mutation {{
                createChatMessage(content: "{}", sender: "{}") {{
                    id
                    content
                    sender
                }}
            }}
        "#, content, sender);

        let res = schema.execute(query).await;
        println!("res: {:?}", res);
        assert!(res.is_ok());

        // Convert to JSON and verify
        let json_str = serde_json::to_string(&res.data).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(json_value["createChatMessage"]["content"], content);
        assert_eq!(json_value["createChatMessage"]["sender"], sender);
    }
}
