#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;

use auth::commands::sign_in;

use std::sync::{Arc, Mutex};

use tauri::Manager;

struct Tokens(Arc<Mutex<TokenStore>>);

struct TokenStore {
    access_token: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: Option<String>,
}

#[tauri::command]
fn get_user_info(window: tauri::Window) -> Result<UserProfile, String> {
    // Access the shared state to get the access token.
    let binding = window.state::<Tokens>();
    let tokens = binding.0.lock().unwrap();
    let access_token = tokens.access_token.clone();

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

fn main() {
    tauri::Builder::default()
        .manage(Tokens(Arc::new(Mutex::new(TokenStore {
            access_token: None,
        }))))
        .invoke_handler(tauri::generate_handler![sign_in, get_user_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
