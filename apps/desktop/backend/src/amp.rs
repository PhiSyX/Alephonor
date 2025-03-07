use crate::{apache, mysql};

#[tauri::command]
pub async fn check_amp_service_status() -> bool
{
	let mut s = apache::check_apache2_service_status().await;
	s &= mysql::check_mysql_service_status().await;
	s
}

#[tauri::command]
pub async fn start_amp_service()
{
	apache::start_apache2_service().await;
	mysql::start_mysql_service().await;
}

#[tauri::command]
pub async fn stop_amp_service()
{
	apache::stop_apache2_service().await;
	mysql::stop_mysql_service().await;
}
