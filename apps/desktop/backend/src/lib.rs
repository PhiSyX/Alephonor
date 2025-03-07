mod account;
mod amp;
mod apache;
mod mysql;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run()
{
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			account::account_info,
			account::post_account_form,
			amp::check_amp_service_status,
			amp::start_amp_service,
			amp::stop_amp_service
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
