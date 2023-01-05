// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        isbn -> Varchar,
        copies_available -> Int4,
        copies -> Int4,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        address -> Varchar,
        age -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    books,
    members,
);
