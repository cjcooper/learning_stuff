// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        primary_author -> Nullable<Varchar>,
    }
}
