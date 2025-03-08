# Chat Message Implementation

This document outlines the implementation of the chat message functionality using SQLite as the database and GraphQL for API interactions.

## Database Schema

The chat message model is implemented with the following schema:

```sql
CREATE TABLE chat_messages (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  content TEXT NOT NULL,
  sender TEXT NOT NULL,
  timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## Model Structure

The chat message model is defined in `src/db/models.rs` with the following structure:

```rust
pub struct ChatMessage {
    pub id: i32,
    pub content: String,
    pub sender: String,
    pub timestamp: NaiveDateTime,
}
```

## GraphQL Operations

### Queries

Two queries are implemented for retrieving chat messages:

1. `chatMessages`: Retrieves all chat messages, ordered by timestamp in descending order.
   ```graphql
   query {
     chatMessages {
       id
       content
       sender
       timestamp
     }
   }
   ```

2. `chatMessage(id: Int!)`: Retrieves a single chat message by its ID.
   ```graphql
   query {
     chatMessage(id: 1) {
       id
       content
       sender
       timestamp
     }
   }
   ```

### Mutation

A mutation is implemented for creating new chat messages:

```graphql
mutation {
  createChatMessage(content: "Hello, world!", sender: "User") {
    id
    content
    sender
    timestamp
  }
}
```

## Implementation Details

- SQLite is used as the database with Diesel ORM for database interactions
- The chat message model implements `SimpleObject` for GraphQL schema generation
- Migrations are automatically run when the server starts
- Tests are included for both queries and mutations
