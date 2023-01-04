// @generated automatically by Diesel CLI.
use diesel::table;

table! {
    members (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        address -> Varchar,
        age -> Int4,
    }
}

table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        isbn -> Varchar,
        copies_available -> Int4,
        copies -> Int4,
    }
}
