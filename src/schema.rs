table! {
    heartbeats (id) {
        id -> Integer,
        entity -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        category -> Text,
        time -> Timestamp,
        project -> Text,
        branch -> Text,
        language -> Text,
        lines -> Integer,
        line_number -> Integer,
        cursor_pos -> Integer,
        is_write -> Bool,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        display_name -> Nullable<Text>,
        api_key -> Nullable<Text>,
        email_confirmed -> Bool,
        website -> Nullable<Text>,
        location -> Nullable<Text>,
        email_public -> Bool,
        logged_time_public -> Bool,
        languages_used_public -> Bool,
        last_heartbeat -> Nullable<Timestamp>,
        registered_time -> Timestamp,
    }
}

joinable!(heartbeats -> users (user_id));

allow_tables_to_appear_in_same_query!(
    heartbeats,
    users,
);
