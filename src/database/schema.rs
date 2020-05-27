table! {
    config (config_name) {
        config_name -> Text,
        config_value -> Text,
    }
}

table! {
    posts (id) {
        id -> Int4,
        draft -> Bool,
        publish_time -> Timestamp,
        slug -> Text,
        title -> Text,
        content -> Text,
        author -> Nullable<Text>,
    }
}

table! {
    users (username) {
        username -> Text,
        display_name -> Text,
        password_salt -> Text,
        password_hash -> Text,
        is_admin -> Bool,
        last_update -> Timestamp,
    }
}

joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(
    config,
    posts,
    users,
);
