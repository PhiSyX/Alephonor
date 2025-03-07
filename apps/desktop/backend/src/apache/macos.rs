use std::time::Duration;

use tokio::process::Command;
use tokio::time::sleep;

const XAMPP_ROOT: &str = "/Applications/XAMPP/xamppfiles";

pub async fn check_apache2_service_status() -> String
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("apache2/scripts/ctl.sh");
	cmd.arg("status");
	let out_apache2 = cmd.output().await.expect("Failed to execute command");
	String::from_utf8(out_apache2.stdout)
		.unwrap()
		.trim()
		.to_owned()
}

pub async fn start_apache2_service() -> String
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("bin/apachectl");
	cmd.args(["-E", "./logs/error_log"]);
	cmd.args(["-k", "start"]);
	_ = cmd.output().await.expect("Failed to execute command");
	sleep(Duration::from_millis(60)).await;
	check_apache2_service_status().await
}

pub async fn stop_apache2_service() -> String
{
	let mut cmd = Command::new("bash");
	cmd.current_dir(XAMPP_ROOT);
	cmd.arg("bin/apachectl");
	cmd.args(["-E", "./logs/error_log"]);
	cmd.args(["-k", "stop"]);
	_ = cmd.output().await.expect("Failed to execute command");
	sleep(Duration::from_millis(60)).await;
	check_apache2_service_status().await
}
