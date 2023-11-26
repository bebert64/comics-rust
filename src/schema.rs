// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "archive_status"))]
    pub struct ArchiveStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "book_type"))]
    pub struct BookType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ArchiveStatus;

    archives (id) {
        id -> Int4,
        path -> Text,
        status -> ArchiveStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BookType;

    books (id) {
        id -> Int4,
        title -> Nullable<Text>,
        path -> Text,
        book_type -> BookType,
        // Number of the TPB inside the volume. Ex: Batman v2 v03 => 3
        volume_number -> Nullable<Int4>,
        volume -> Nullable<Int4>,
    }
}

diesel::table! {
    books__additional_files (id) {
        id -> Int4,
        book_id -> Int4,
        file_path -> Text,
        position -> Int4,
    }
}

diesel::table! {
    books__issues (id) {
        id -> Int4,
        book_id -> Int4,
        issue_id -> Int4,
        position -> Int4,
    }
}

diesel::table! {
    issues (id) {
        id -> Int4,
        volume_id -> Int4,
        number -> Int4,
        path -> Nullable<Text>,
    }
}

diesel::table! {
    reading_orders (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    reading_orders__books (id) {
        id -> Int4,
        book_id -> Int4,
        reading_order_id -> Int4,
        position -> Int4,
    }
}

diesel::table! {
    volumes (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(books__additional_files -> books (book_id));
diesel::joinable!(books__issues -> books (book_id));
diesel::joinable!(books__issues -> issues (issue_id));
diesel::joinable!(issues -> volumes (volume_id));
diesel::joinable!(reading_orders__books -> books (book_id));
diesel::joinable!(reading_orders__books -> reading_orders (reading_order_id));

diesel::allow_tables_to_appear_in_same_query!(
    archives,
    books,
    books__additional_files,
    books__issues,
    issues,
    reading_orders,
    reading_orders__books,
    volumes,
);
