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

pub fn get_token_store(window: tauri::Window) -> TokenStore {
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
    tokens.flush(window.app_handle());

    // Emit a sign out event
    window
        .emit("sign_out_complete", None::<()>)
        .expect("Failed to emit sign-out event");
}

#[tauri::command]
fn check_if_signed_in(window: tauri::Window) -> bool {
    let token_store = get_token_store(window);

    let response = token_store.access_token.is_some();

    response
}

fn main() {
    tauri::Builder::default()
        .manage(Tokens::new())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            sign_in,
            sign_out,
            check_if_signed_in,
            get_user_info
        ])
        .setup(|app| {
            let tokens = app.state::<Tokens>();
            let mut tokens = tokens.lock();

            tokens.load_tokens(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
