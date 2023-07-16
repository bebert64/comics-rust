// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "zip_status"))]
    pub struct ZipStatus;
}

diesel::table! {
    books (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    books_additional_files (id) {
        id -> Int4,
        bookd_id -> Int4,
        file_path -> Text,
    }
}

diesel::table! {
    books_issues (id) {
        id -> Int4,
        bookd_id -> Int4,
        issue_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ZipStatus;

    issues (id) {
        id -> Int4,
        volume_id -> Int4,
        number -> Int4,
        dir -> Nullable<Text>,
        status -> ZipStatus,
    }
}

diesel::table! {
    reading_order_elements (id) {
        id -> Int4,
        issue_id -> Nullable<Int4>,
        book_id -> Nullable<Int4>,
        reading_order_id -> Nullable<Int4>,
    }
}

diesel::table! {
    reading_orders (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    volumes (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ZipStatus;

    zips (id) {
        id -> Int4,
        path -> Text,
        status -> ZipStatus,
    }
}

diesel::joinable!(books_additional_files -> books (bookd_id));
diesel::joinable!(books_issues -> books (bookd_id));
diesel::joinable!(books_issues -> issues (issue_id));
diesel::joinable!(issues -> volumes (volume_id));
diesel::joinable!(reading_order_elements -> books (book_id));
diesel::joinable!(reading_order_elements -> issues (issue_id));
diesel::joinable!(reading_order_elements -> reading_orders (reading_order_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    books_additional_files,
    books_issues,
    issues,
    reading_order_elements,
    reading_orders,
    volumes,
    zips,
);
