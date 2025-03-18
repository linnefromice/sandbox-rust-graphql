# sandbox-rust-graphql

```bash
$ cargo run
GraphQL playground: http://localhost:8000/graphql
```

```graphql
mutation MutationCreateChatMessage {
  createChatMessage(
    content: "5th Content",
    sender: "Paruu"
  ) {
    id
    content
    sender
    timestamp
  }
}

query QueryChats {
  chatMessages(first: 3) {
    edges {
    	node {
        id
        content
        sender
        timestamp
      }
      cursor
    }
    pageInfo {
      hasNextPage
      endCursor
    }
  }
}

query QueryChat {
  chatMessage(id: 3) {
    id
    content
    sender
    timestamp
  }
}
```

## TODOs

- Pagination by specifying after attribute in query fails
