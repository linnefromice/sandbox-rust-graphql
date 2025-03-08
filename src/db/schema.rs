// @generated automatically by Diesel CLI.

diesel::table! {
    chat_messages (id) {
        id -> Integer,
        content -> Text,
        sender -> Text,
        timestamp -> Timestamp,
    }
}
