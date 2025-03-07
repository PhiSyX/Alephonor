use std::time::Duration;

use tokio::process::Command;
use tokio::time::sleep;

const XAMPP_ROOT: &str = "/Applications/XAMPP/xamppfiles";

pub async fn check_mysql_service_status() -> String
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("mysql/scripts/ctl.sh");
	cmd.arg("status");
	let out = cmd.output().await.expect("Failed to execute command");
	String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

pub async fn start_mysql_service() -> String
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("mysql/scripts/ctl.sh");
	cmd.arg("start");
	_ = cmd.output().await.expect("Failed to execute command");
	sleep(Duration::from_millis(60)).await;
	check_mysql_service_status().await
}

pub async fn stop_mysql_service() -> String
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("mysql/scripts/ctl.sh");
	cmd.arg("stop");
	_ = cmd.output().await.expect("Failed to execute command");
	sleep(Duration::from_millis(60)).await;
	check_mysql_service_status().await
}
