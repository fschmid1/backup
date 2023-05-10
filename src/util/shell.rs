pub fn execute(command: String, args: Vec<String>) -> bool {
    let mut cmd = std::process::Command::new("bash");
    cmd.args(["-c".to_string(), format!("{} {}", command, args.join(" "))]);
    let output = cmd.output().expect("failed to execute process");
    return output.status.success();
}
