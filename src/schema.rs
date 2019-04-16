table! {
    heartbeats (id) {
        id -> Nullable<Integer>,
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
        id -> Nullable<Integer>,
        email -> Text,
        display_name -> Nullable<Text>,
        api_key -> Nullable<Text>,
        email_confirmed -> Bool,
        website -> Nullable<Text>,
        location -> Nullable<Text>,
        last_heartbeat -> Nullable<Timestamp>,
        email_public -> Bool,
        logged_time_public -> Bool,
        languages_used_public -> Bool,
        registered_time -> Timestamp,
        last_active_time -> Timestamp,
    }
}

joinable!(heartbeats -> users (user_id));

allow_tables_to_appear_in_same_query!(heartbeats, users,);
