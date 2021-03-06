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
        author -> Text,
    }
}

table! {
    sessions (session_id) {
        session_id -> Text,
        username -> Text,
        expiry -> Timestamp,
    }
}

table! {
    tags (tag_name, post_id) {
        tag_name -> Text,
        post_id -> Int4,
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
joinable!(sessions -> users (username));
joinable!(tags -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    config,
    posts,
    sessions,
    tags,
    users,
);
