// @generated automatically by Diesel CLI.

diesel::table! {
    configs (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        user -> Nullable<Varchar>,
        content -> Mediumtext,
        created_at -> Datetime,
        modified_at -> Datetime,
    }
}