#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod store;

use auth::auth_commands::{check_if_signed_in, sign_in, sign_out};
use auth::data_commands::get_user_info;

use tauri::Manager;

use store::tokens::{TokenStore, Tokens};

pub fn get_token_store(window: tauri::Window) -> TokenStore {
    // Access the shared state to get token store
    let binding = window.state::<Tokens>();
    let tokens = binding.lock();

    // Get a copy of the tokenstore data
    let token_store = tokens.clone();

    return token_store;
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
