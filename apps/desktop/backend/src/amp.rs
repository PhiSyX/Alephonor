use crate::{apache, mysql};

#[tauri::command]
pub async fn check_amp_service_status() -> bool
{
	mysql::check_mysql_service_status().await
		&& apache::check_apache2_service_status().await
}

#[tauri::command]
pub async fn start_amp_service()
{
	mysql::start_mysql_service().await;
	apache::start_apache2_service().await;
}

#[tauri::command]
pub async fn stop_amp_service()
{
	mysql::stop_mysql_service().await;
	apache::stop_apache2_service().await;
}
