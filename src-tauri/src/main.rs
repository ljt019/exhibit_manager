#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod store;

use auth::commands::sign_in;

use tauri::Manager;

use store::tokens::{TokenStore, Tokens};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: Option<String>,
}

fn get_token_store(window: tauri::Window) -> TokenStore {
    // Access the shared state to get token store
    let binding = window.state::<Tokens>();
    let tokens = binding.lock();

    // Get a copy of the tokenstore data
    let token_store = tokens.clone();

    return token_store;
}

#[tauri::command]
fn get_user_info(window: tauri::Window) -> Result<UserProfile, String> {
    let token_store = get_token_store(window);

    let access_token = token_store.access_token;

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

#[tauri::command]
fn sign_out(window: tauri::Window) {
    // Access the shared state to get token store
    let binding = window.state::<Tokens>();
    let mut tokens = binding.lock();

    // Flush the token store
    tokens.flush();

    // Emit a sign out event
    window
        .emit("sign_out_complete", None::<()>)
        .expect("Failed to emit sign-out event");
}

fn main() {
    tauri::Builder::default()
        .manage(Tokens::new())
        .invoke_handler(tauri::generate_handler![sign_in, sign_out, get_user_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
