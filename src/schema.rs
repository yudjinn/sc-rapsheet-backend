// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        password -> Varchar,
        name -> Varchar,
        active -> Bool,
    }
}
