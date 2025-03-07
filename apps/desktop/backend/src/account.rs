#[derive(serde::Deserialize)]
struct AccountInfoEnv
{
	user_account_avatar: String,
	user_account_name: String,
	user_account_fullname: String,
	user_account_email: String,
	user_account_color: String,
}

#[derive(serde::Serialize)]
pub struct AccountInfo
{
	avatar: String,
	email: String,
	fullname: String,
	name: String,
	preferred_color: String,
}

#[tauri::command]
pub fn account_info() -> AccountInfo
{
	// TODO: remplacer par les vraies informations de l'OS
	let env: AccountInfoEnv = alephonor_env::from_file(".env.local")
		.expect("You need to have a .env.local file to the backend directory");

	AccountInfo {
		avatar: env.user_account_avatar,
		email: env.user_account_email,
		fullname: env.user_account_fullname,
		name: env.user_account_name,
		preferred_color: env.user_account_color,
	}
}

#[tauri::command]
pub fn post_account_form(raw_password: &str) -> bool
{
	raw_password == "azerty"
}
