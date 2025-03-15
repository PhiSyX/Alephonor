use std::time::Duration;
use tokio::process::Command;

const XAMPP_ROOT: &str = "/Applications/XAMPP/xamppfiles";

pub async fn check_apache2_service_status() -> bool
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("apache2/scripts/ctl.sh");
	cmd.arg("status");
	let out_apache2 = cmd.output().await.expect("Failed to execute command");
	String::from_utf8(out_apache2.stdout)
		.unwrap()
		.contains("apache already running")
}

pub async fn start_apache2_service()
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("bin/apachectl");
	cmd.args(["-E", "./logs/error_log"]);
	cmd.args(["-k", "start"]);
	_ = cmd.output().await.expect("Failed to execute command");
	tokio::time::sleep(Duration::from_millis(250)).await;
}

pub async fn stop_apache2_service()
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("bin/apachectl");
	cmd.args(["-E", "./logs/error_log"]);
	cmd.args(["-k", "stop"]);
	_ = cmd.output().await.expect("Failed to execute command");
}
