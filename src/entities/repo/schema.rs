// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Text,
        thumbnail -> Nullable<Binary>,
        is_tpb -> Bool,
    }
}

diesel::table! {
    comics (id) {
        id -> Integer,
        title -> Text,
        thumbnail -> Nullable<Binary>,
        publisher_id -> Nullable<Integer>,
        comic_vine_id -> Nullable<Integer>,
    }
}

diesel::table! {
    creators (id) {
        id -> Integer,
        name -> Text,
        thumbnail -> Nullable<Binary>,
        comic_vine_id -> Nullable<Integer>,
    }
}

diesel::table! {
    issues (id) {
        id -> Integer,
        is_read -> Bool,
        number -> Integer,
        cover_date -> Nullable<Date>,
        thumbnail -> Nullable<Binary>,
        volume_id -> Nullable<Integer>,
        comic_vine_id -> Nullable<Integer>,
        book_id -> Nullable<Integer>,
        author_id -> Nullable<Integer>,
        artist_id -> Nullable<Integer>,
    }
}

diesel::table! {
    publishers (id) {
        id -> Integer,
        name -> Text,
        thumbnail -> Nullable<Binary>,
        comic_vine_id -> Nullable<Integer>,
    }
}

diesel::table! {
    story_arcs (id) {
        id -> Integer,
        title -> Text,
        thumbnail -> Nullable<Binary>,
        comic_vine_id -> Nullable<Integer>,
    }
}

diesel::table! {
    volumes (id) {
        id -> Integer,
        number -> Integer,
        thumbnail -> Nullable<Binary>,
        comic_id -> Integer,
        comic_vine_id -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    books,
    comics,
    creators,
    issues,
    publishers,
    story_arcs,
    volumes,
);
