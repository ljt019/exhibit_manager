#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod token_manager;

use auth::auth_commands::{check_if_signed_in, sign_in, sign_out};
use auth::data_commands::get_user_info;

use tauri::Manager;

use token_manager::tokens::TokenManager;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            let token_manager = TokenManager::new(app.handle().clone());
            token_manager.load_tokens();

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
