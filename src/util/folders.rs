use crate::db::{models::BackupJob, wrapper::update_job};

fn create_ifnot_exitsts(folder: String, ssh: &Option<String>) {
    if let Some(ssh_arg) = ssh {
        super::shell::execute(
            "ssh".to_string(),
            [format!(
                "{} '{}'",
                ssh_arg,
                format!("'mkdir -p {}'", folder)
            )]
            .to_vec(),
        );
    } else {
        super::shell::execute("mkdir".to_string(), ["-p".to_string(), folder].to_vec());
    }
}

fn create_hourly(target_folder: String, ssh: Option<String>) {
    create_ifnot_exitsts(format!("{}/hourly", target_folder), &ssh);
    for i in 1..25 {
        create_ifnot_exitsts(format!("{}/hourly/{}", target_folder, i), &ssh);
    }
}

fn create_monthly(target_folder: String, ssh: Option<String>) {
    create_ifnot_exitsts(format!("{}/monthly", target_folder), &ssh);
    for i in 1..13 {
        let ssh = ssh.clone();
        create_ifnot_exitsts(format!("{}/monthly/{}", target_folder, i), &ssh);
    }
}

fn create_weekly(target_folder: String, ssh: Option<String>) {
    create_ifnot_exitsts(format!("{}/weekly", target_folder), &ssh);
    for i in 1..6 {
        let ssh = ssh.clone();
        create_ifnot_exitsts(format!("{}/weekly/{}", target_folder, i), &ssh);
    }
}

fn create_daily(target_folder: String, ssh: Option<String>) {
    create_ifnot_exitsts(format!("{}/daily", target_folder), &ssh);
    for i in 1..8 {
        let ssh = ssh.clone();
        create_ifnot_exitsts(format!("{}/daily/{}", target_folder, i), &ssh);
    }
}

fn create_folders_structure(target_folder: String, ssh: Option<String>) {
    create_hourly(target_folder.clone(), ssh.clone());
    create_daily(target_folder.clone(), ssh.clone());
    create_weekly(target_folder.clone(), ssh.clone());
    create_monthly(target_folder.clone(), ssh.clone());
}

pub async fn create_folders(backup_job: BackupJob) {
    tokio::spawn(async move {
        let ssh_arg: Option<String>;
        let folder: String;
        let mut backup_job = backup_job.clone();
        if backup_job.dst.contains(":") && backup_job.dst.contains("@") {
            ssh_arg = Some(backup_job.dst.split(":").collect::<Vec<&str>>()[0].to_string());
            folder = backup_job.dst.split(":").collect::<Vec<&str>>()[1].to_string();
        } else {
            ssh_arg = None;
            folder = backup_job.dst.clone();
        }
        crate::util::folders::create_folders_structure(folder, ssh_arg);
        backup_job.is_ready = true;
        let _ = update_job(backup_job).await;
    });
}
