use tauri::State;

#[derive(serde::Deserialize)]
pub struct AccountInfoEnv
{
	user_account_avatar: String,
	user_account_name: String,
	user_account_fullname: String,
	user_account_email: String,
	user_account_color: Option<String>,
	user_account_pass: String,
}

#[derive(serde::Serialize)]
pub struct AccountInfo
{
	avatar: String,
	email: String,
	fullname: String,
	name: String,
	preferred_color: Option<String>,
}

pub fn get_env_account_info() -> AccountInfoEnv
{
	return alephonor_env::from_file(".env.local")
		.expect("You need to have a .env.local file to the backend directory");
}

#[tauri::command]
pub fn account_info(env: State<'_, AccountInfoEnv>) -> AccountInfo
{
	AccountInfo {
		avatar: env.user_account_avatar.clone(),
		email: env.user_account_email.clone(),
		fullname: env.user_account_fullname.clone(),
		name: env.user_account_name.clone(),
		preferred_color: env.user_account_color.clone(),
	}
}

#[tauri::command]
pub fn post_account_form(
	raw_password: &str,
	env: State<'_, AccountInfoEnv>,
) -> bool
{
	env.user_account_pass.eq(raw_password)
}
