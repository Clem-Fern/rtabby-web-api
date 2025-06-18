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

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        user_id -> Text,
        platform -> Text,
        token -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(configs, users,);
