diesel::table! {
    backup_jobs (id) {
        id -> Text,
        name -> Text,
        src -> Text,
        dst -> Text,
        hourly -> Bool,
        daily -> Bool,
        weekly -> Bool,
        monthly -> Bool,
    }
}
