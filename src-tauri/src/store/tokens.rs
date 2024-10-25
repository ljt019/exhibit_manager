use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

pub struct Tokens(pub Arc<Mutex<TokenStore>>);

impl Tokens {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(TokenStore::new())))
    }

    pub fn lock(&self) -> std::sync::MutexGuard<'_, TokenStore> {
        self.0.lock().expect("Failed to lock TokenStore")
    }
}

pub struct Channels {
    pub tx: std::sync::mpsc::Sender<String>,
    pub rx: Arc<Mutex<std::sync::mpsc::Receiver<String>>>,
}

impl Clone for Channels {
    fn clone(&self) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}

#[derive(Clone)]
pub struct TokenStore {
    pub oauth_client: Option<oauth2::basic::BasicClient>,
    pub channel: Option<Channels>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            oauth_client: None,
            channel: None,
            access_token: None,
            refresh_token: None,
            expires_at: None,
        }
    }

    pub fn get_channel(
        &self,
    ) -> (
        std::sync::mpsc::Sender<String>,
        std::sync::mpsc::Receiver<String>,
    ) {
        todo!()
    }

    pub fn save_tokens(&mut self, app_handle: tauri::AppHandle) {
        // Get path to the store
        let appdata_local = tauri::api::path::app_local_data_dir(&app_handle.config()).unwrap();
        let store_path = appdata_local.join("tokens.json");

        // Create a new token store from tauri-plugin-store
        let mut store = tauri_plugin_store::StoreBuilder::new(app_handle, store_path).build();

        let access_token = self.access_token.clone().expect("No access token set");
        let refresh_token = self.refresh_token.clone().expect("No refresh token set");
        let expires_at = self.expires_at.clone().expect("No expires_at set");

        // Save the tokens to the store
        store
            .insert(
                "access_token".to_string(),
                serde_json::Value::String(access_token),
            )
            .expect("Failed to insert tokens into store");
        store
            .insert(
                "refresh_token".to_string(),
                serde_json::Value::String(refresh_token),
            )
            .expect("Failed to insert tokens into store");
        store
            .insert(
                "expires_at".to_string(),
                serde_json::Value::String(expires_at.to_rfc3339()),
            )
            .expect("Failed to insert tokens into store");

        // Save the store
        store.save().expect("Failed to save token store");
    }

    pub fn load_tokens(&mut self, app_handle: tauri::AppHandle) {
        // Get path to the store
        let appdata_local = tauri::api::path::app_local_data_dir(&app_handle.config()).unwrap();
        let store_path = appdata_local.join("tokens.json");

        // Create a new token store from tauri-plugin-store
        let mut store = tauri_plugin_store::StoreBuilder::new(app_handle, store_path).build();

        store.load().expect("Failed to load token store");

        // Load the tokens from the store
        let access_token = store
            .get("access_token")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let refresh_token = store
            .get("refresh_token")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let expires_at = store.get("expires_at").and_then(|v| {
            v.as_str().map(|s| {
                DateTime::parse_from_rfc3339(s)
                    .expect("Failed to parse expires_at")
                    .with_timezone(&Utc)
            })
        });

        // Set the tokens
        self.access_token = access_token;
        self.refresh_token = refresh_token;
        self.expires_at = expires_at;
    }

    pub fn flush(&mut self, app_handle: tauri::AppHandle) {
        self.oauth_client = None;
        self.access_token = None;
        self.refresh_token = None;
        self.expires_at = None;

        // Get path to the store
        let appdata_local = tauri::api::path::app_local_data_dir(&app_handle.config()).unwrap();
        let store_path = appdata_local.join("tokens.json");

        // Create a new token store from tauri-plugin-store
        let mut store = tauri_plugin_store::StoreBuilder::new(app_handle, store_path).build();

        // Wipe the store
        store.clear().expect("Failed to clear token store");
        store.save().expect("Failed to save token store");
    }
}
