// @generated automatically by Diesel CLI.

diesel::table! {
    configs (id) {
        id -> Integer,
        name -> Text,
        user -> Nullable<Text>,
        shared -> Nullable<Integer>,
        share_hotkey -> Bool,
        share_windows_settings -> Bool,
        share_vault -> Bool,
        content -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}
