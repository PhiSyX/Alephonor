mod account;
mod amp;
mod apache;
mod apps;
mod mysql;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run()
{
	let env_account_info = account::get_env_account_info();

	tauri::Builder::default()
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_os::init())
		.plugin(tauri_plugin_shell::init())
		.manage(env_account_info)
		.invoke_handler(tauri::generate_handler![
			account::account_info,
			account::post_account_form,
			apps::all_services,
			amp::check_amp_service_status,
			amp::start_amp_service,
			amp::stop_amp_service
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
