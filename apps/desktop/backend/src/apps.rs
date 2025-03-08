use tauri::Manager;
use tauri::path::BaseDirectory;
use tauri_plugin_fs::FsExt;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Service
{
	name: String,
	title: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	image: Option<String>,
	installed: bool,
	commands: ServiceCommands,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServiceCommands
{
	install: String,
	check: String,
	start: String,
	stop: String,
}

#[tauri::command]
pub async fn all_services(app_handle: tauri::AppHandle) -> Vec<Service>
{
	let Ok(services_file) = app_handle
		.path()
		.resolve("services.json", BaseDirectory::AppData)
	else {
		return vec![];
	};

	let Ok(fd) = app_handle.fs().read(services_file) else {
		return vec![];
	};

	let Ok(services) = serde_json::from_slice(&fd) else {
		return vec![];
	};

	services
}
