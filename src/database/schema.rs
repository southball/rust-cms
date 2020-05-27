table! {
    posts (id) {
        id -> Int4,
        draft -> Bool,
        publish_time -> Timestamp,
        slug -> Text,
        title -> Text,
        content -> Text,
        author -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        display_name -> Text,
        password_salt -> Text,
        password_hash -> Text,
    }
}

joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(posts, users,);
