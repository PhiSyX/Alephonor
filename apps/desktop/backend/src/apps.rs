use tokio::fs::File;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Service
{
	name: String,
	title: String,
	commands: ServiceCommands,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServiceCommands
{
	check: String,
	start: String,
	stop: String,
}

#[tauri::command]
pub async fn all_services() -> Vec<Service>
{
	let Ok(fd) = File::open("./testdata/services.json").await else {
		return vec![];
	};

	let Ok(services) = serde_json::from_reader(fd.into_std().await) else {
		return vec![];
	};

	services
}
