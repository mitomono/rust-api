// @generated automatically by Diesel CLI.
use diesel::table;

table! {
    employees (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        department -> Varchar,
        salary -> Float8,
        age -> Int4,
    }
}
