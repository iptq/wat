table! {
    heartbeats (id) {
        id -> Int4,
        user_id -> Int4,
        entity -> Varchar,
        entity_type -> Varchar,
        category -> Nullable<Varchar>,
        time -> Timestamp,
        project -> Nullable<Varchar>,
        branch -> Nullable<Varchar>,
        language -> Nullable<Varchar>,
        dependencies -> Nullable<Varchar>,
        lines -> Int4,
        line_number -> Nullable<Int4>,
        cursor_pos -> Nullable<Int4>,
        is_write -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        display_name -> Nullable<Varchar>,
        api_key -> Varchar,
        email_confirmed -> Bool,
        website -> Nullable<Varchar>,
        location -> Nullable<Varchar>,
        email_public -> Bool,
        logged_time_public -> Bool,
        languages_used_public -> Bool,
        last_heartbeat -> Nullable<Timestamp>,
        registered_time -> Timestamp,
    }
}

joinable!(heartbeats -> users (user_id));

allow_tables_to_appear_in_same_query!(heartbeats, users,);
