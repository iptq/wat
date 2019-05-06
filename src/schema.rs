table! {
    heartbeats (id) {
        id -> Integer,
        user_id -> Integer,
        entity -> Text,
        entity_type -> Text,
        category -> Nullable<Text>,
        time -> Timestamp,
        project -> Nullable<Text>,
        branch -> Nullable<Text>,
        language -> Nullable<Text>,
        dependencies -> Nullable<Text>,
        lines -> Integer,
        line_number -> Nullable<Integer>,
        cursor_pos -> Nullable<Text>,
        is_write -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        display_name -> Nullable<Text>,
        api_key -> Text,
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

allow_tables_to_appear_in_same_query!(heartbeats, users,);
