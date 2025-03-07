mod schema;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use schema::AppSchema;
use std::net::SocketAddr;

async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    // Create the schema
    let schema = schema::create_schema();

    // Build our application with routes
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .with_state(schema);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("GraphQL playground: http://localhost:8000/graphql");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::Request;

    #[tokio::test]
    async fn test_hello_query_integration() {
        let schema = schema::create_schema();
        let query = r#"
            query {
                hello
            }
        "#;
        
        let request = Request::new(query);
        let response = schema.execute(request).await;
        
        // Convert to JSON string and then parse with serde_json for assertion
        let json_str = serde_json::to_string(&response.data).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(json_value["hello"], "hello");
    }
}
