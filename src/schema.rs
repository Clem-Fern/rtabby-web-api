// @generated automatically by Diesel CLI.

diesel::table! {
    configs (id) {
        id -> Integer,
        name -> Text,
        user -> Nullable<Text>,
        content -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}