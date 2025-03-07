use crate::{apache, mysql};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AmpStatus {
	apache2: String,
	mysql: String,
}

#[tauri::command]
pub async fn check_amp_service_status() -> AmpStatus {
	AmpStatus {
		apache2: apache::check_apache2_service_status().await,
		mysql: mysql::check_mysql_service_status().await,
	}
}

#[tauri::command]
pub async fn start_amp_service() -> AmpStatus {
	AmpStatus {
		apache2: apache::start_apache2_service().await,
		mysql: mysql::start_mysql_service().await,
	}
}

#[tauri::command]
pub async fn stop_amp_service() -> AmpStatus {
	AmpStatus {
		apache2: apache::stop_apache2_service().await,
		mysql: mysql::stop_mysql_service().await,
	}
}
