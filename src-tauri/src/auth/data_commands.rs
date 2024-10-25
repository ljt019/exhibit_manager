use crate::token_manager::tokens::TokenManager;
use tauri::Manager;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: Option<String>,
}

#[tauri::command]
pub fn get_user_info(window: tauri::Window) -> Result<UserProfile, String> {
    let token_manager = window.state::<TokenManager>();

    let access_token = token_manager.get_token_data().access_token;

    if let Some(access_token) = access_token {
        // Use the access token to fetch user info.
        let user_profile = reqwest::blocking::Client::new()
            .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .send()
            .map_err(|e| format!("Failed to fetch user profile: {}", e))?
            .json::<UserProfile>()
            .map_err(|e| format!("Failed to parse user profile JSON: {}", e))?;

        Ok(user_profile)
    } else {
        Err("No access token found".to_string())
    }
}
