#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod store;

use auth::auth_commands::{check_if_signed_in, sign_in, sign_out};
use auth::data_commands::get_user_info;

use tauri::Manager;

use store::tokens::{OAuthTokenStore, TokenManager};

pub fn get_token_store(window: tauri::Window) -> OAuthTokenStore {
    // Access the shared state to get token store
    let binding = window.state::<TokenManager>();
    let tokens = binding.lock_store();

    // Get a copy of the tokenstore data
    let token_store = tokens.clone();

    return token_store;
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let token_manager = TokenManager::new(app_handle.clone());
            token_manager.load_persisted_tokens();
            app.manage(token_manager);

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            sign_in,
            sign_out,
            check_if_signed_in,
            get_user_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
