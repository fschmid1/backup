// @generated automatically by Diesel CLI.

diesel::table! {
    backup_jobs (id) {
        id -> Text,
        name -> Text,
        src -> Text,
        dst -> Text,
        is_ready -> Bool,
        hourly -> Bool,
        daily -> Bool,
        weekly -> Bool,
        monthly -> Bool,
    }
}

diesel::table! {
    logs (id) {
        id -> Nullable<Integer>,
        job_id -> Nullable<Text>,
        level -> Integer,
        message -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(logs -> backup_jobs (job_id));

diesel::allow_tables_to_appear_in_same_query!(
    backup_jobs,
    logs,
);
