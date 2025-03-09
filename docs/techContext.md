# Technical Context

## Technologies Used
- **async-graphql**: GraphQL implementation
- **axum**: Web framework
- **diesel**: ORM for database interactions
- **SQLite**: Database engine
- **Tokio**: Async runtime

## Development Setup
1. Install Rust toolchain
2. Install diesel_cli for database migrations
3. Set up SQLite database
4. Configure environment variables in .env file

## Technical Constraints
- SQLite database limits scalability
- Single-threaded async runtime
- Limited connection pooling
- No built-in authentication

## Dependencies
- async-graphql = "4.0"
- axum = "0.6"
- diesel = { version = "2.0", features = ["sqlite"] }
- tokio = { version = "1.0", features = ["full"] }
- dotenv = "0.15"

## Database Configuration
- Database URL: DATABASE_URL in .env
- Migration files in migrations/ directory
- Schema defined in src/db/schema.rs
- Models defined in src/db/models.rs

## API Endpoints
- GraphQL endpoint: /graphql
- GraphiQL playground: /graphiql
