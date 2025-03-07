use tokio::process::Command;

const XAMPP_ROOT: &str = "/Applications/XAMPP/xamppfiles";

pub async fn check_mysql_service_status() -> bool
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("mysql/scripts/ctl.sh");
	cmd.arg("status");
	let out = cmd.output().await.expect("Failed to execute command");
	String::from_utf8(out.stdout)
		.unwrap()
		.contains("mysql already running")
}

pub async fn start_mysql_service()
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("mysql/scripts/ctl.sh");
	cmd.arg("start");
	_ = cmd.output().await.expect("Failed to execute command");
}

pub async fn stop_mysql_service()
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("mysql/scripts/ctl.sh");
	cmd.arg("stop");
	_ = cmd.output().await.expect("Failed to execute command");
}
