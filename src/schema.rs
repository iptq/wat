table! {
    heartbeats (id) {
        id -> Int4,
        entity -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        category -> Varchar,
        time -> Timestamp,
        project -> Varchar,
        branch -> Varchar,
        language -> Varchar,
        lines -> Int4,
        line_number -> Int4,
        cursor_pos -> Int4,
        is_write -> Bool,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        display_name -> Nullable<Varchar>,
        api_key -> Nullable<Varchar>,
        email_confirmed -> Bool,
        website -> Nullable<Varchar>,
        location -> Nullable<Varchar>,
        last_heartbeat -> Nullable<Timestamp>,
        email_public -> Bool,
        logged_time_public -> Bool,
        languages_used_public -> Bool,
        registered_time -> Timestamp,
        last_active_time -> Timestamp,
    }
}

joinable!(heartbeats -> users (user_id));

allow_tables_to_appear_in_same_query!(
    heartbeats,
    users,
);
