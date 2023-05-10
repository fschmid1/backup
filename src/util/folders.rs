fn create_ifnot_exitsts(folder: String, user: &String, server: &String) {
    let mut cmd = std::process::Command::new("ssh");
    cmd.args([
        format!("{}@{}", user, server),
        format!("\"mkdir -p {}\"", folder),
    ]);
    cmd.output().expect("failed to execute process");
}

fn create_hourly(target_folder: String, user: String, server: String) {
    create_ifnot_exitsts(format!("{}/hourly", target_folder), &user, &server);
    for i in 1..25 {
        create_ifnot_exitsts(format!("{}/hourly/{}", target_folder, i), &user, &server);
    }
}

fn create_monthly(target_folder: String, user: String, server: String) {
    create_ifnot_exitsts(format!("{}/monthly", target_folder), &user, &server);
    for i in 1..13 {
        let user = user.clone();
        let server = server.clone();
        create_ifnot_exitsts(format!("{}/monthly/{}", target_folder, i), &user, &server);
    }
}

fn create_weekly(target_folder: String, user: String, server: String) {
    create_ifnot_exitsts(format!("{}/weekly", target_folder), &user, &server);
    for i in 1..6 {
        let user = user.clone();
        let server = server.clone();
        create_ifnot_exitsts(format!("{}/weekly/{}", target_folder, i), &user, &server);
    }
}

fn create_daily(target_folder: String, user: String, server: String) {
    create_ifnot_exitsts(format!("{}/daily", target_folder), &user, &server);
    for i in 1..8 {
        let user = user.clone();
        let server = server.clone();
        create_ifnot_exitsts(format!("{}/daily/{}", target_folder, i), &user, &server);
    }
}

pub fn create_folders(target_folder: String, user: String, server: String) {
    create_hourly(target_folder.clone(), user.clone(), server.clone());
    create_daily(target_folder.clone(), user.clone(), server.clone());
    create_weekly(target_folder.clone(), user.clone(), server.clone());
    create_monthly(target_folder.clone(), user.clone(), server.clone());
}
