// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        author_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

diesel::joinable!(books -> authors (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    notifications,
);
