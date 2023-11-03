// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
