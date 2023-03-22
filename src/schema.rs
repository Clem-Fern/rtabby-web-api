// @generated automatically by Diesel CLI.

diesel::table! {
    configs (id) {
        id -> Integer,
        name -> Varchar,
        user -> Nullable<Varchar>,
        content -> Text,
        created_at -> Datetime,
        modified_at -> Datetime,
    }
}

diesel::table! {
    user_configs (config_id, user) {
        config_id -> Integer,
        user -> Varchar,
        content -> Text,
    }
}

diesel::joinable!(user_configs -> configs (config_id));

diesel::allow_tables_to_appear_in_same_query!(
    configs,
    user_configs,
);
