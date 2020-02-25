table! {
    log_entry (id) {
        id -> Int4,
        hours -> Float4,
        time -> Timestamp,
        user_id -> Int4,
        project_id -> Nullable<Int4>,
    }
}

table! {
    project (id) {
        id -> Int4,
        name -> Text,
        user_id -> Int4,
    }
}

joinable!(log_entry -> project (project_id));

allow_tables_to_appear_in_same_query!(
    log_entry,
    project,
);
