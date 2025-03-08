use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::chat_messages;

#[derive(SimpleObject, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = chat_messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ChatMessage {
    pub id: i32,
    pub content: String,
    pub sender: String,
    #[graphql(name = "timestamp")]
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = chat_messages)]
pub struct NewChatMessage {
    pub content: String,
    pub sender: String,
    pub timestamp: NaiveDateTime,
}
