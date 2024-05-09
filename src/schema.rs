// @generated automatically by Diesel CLI.

diesel::table! {
    notifications (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}
