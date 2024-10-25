use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

/* Constants */
const CLIENT_ID: &str = "883531815886-cr4nl5k7v4hf81bhmfjhe39j3r2ic5lm.apps.googleusercontent.com";
const CLIENT_SECRET: &str = "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";
const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
const TOKEN_URL: &str = "https://accounts.google.com/o/oauth2/token";
const REVOCATION_URL: &str = "https://oauth2.googleapis.com/revoke";

#[derive(Debug, Clone)]
pub struct TokenData {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub struct TokenManager {
    store: Arc<Mutex<OAuthTokenStore>>,
    persisted_store: Arc<Mutex<PersistedStore>>,
}

impl TokenManager {
    pub fn new(app_handle: AppHandle) -> Self {
        let store = OAuthTokenStore::new();
        let persisted_store = PersistedStore::new(app_handle.clone());
        Self {
            store: Arc::new(Mutex::new(store)),
            persisted_store: Arc::new(Mutex::new(persisted_store)),
        }
    }

    pub fn lock_store(&self) -> std::sync::MutexGuard<'_, OAuthTokenStore> {
        self.store.lock().expect("Failed to lock OAuthTokenStore")
    }

    pub fn lock_persisted_store(&self) -> std::sync::MutexGuard<'_, PersistedStore> {
        self.persisted_store
            .lock()
            .expect("Failed to lock PersistedStore")
    }

    pub fn save_persisted_tokens(&self) {
        let store = self.lock_store();
        let token_data = store.get_token_data();
        let mut persisted_store = self.lock_persisted_store();
        persisted_store.save_tokens(&token_data);
    }

    pub fn load_persisted_tokens(&self) {
        let mut persisted_store = self.lock_persisted_store();
        let token_data = persisted_store.load_tokens();
        let mut store = self.lock_store();
        store.set_token_data(token_data);
    }

    pub fn flush(&self) {
        let mut store = self.lock_store();
        store.flush();

        let mut persisted_store = self.lock_persisted_store();
        persisted_store.flush();
    }
}

#[derive(Clone)]
pub struct OAuthTokenStore {
    pub oauth_client: oauth2::basic::BasicClient,
    token_data: TokenData,
}

impl OAuthTokenStore {
    pub fn new() -> Self {
        let client = oauth2::basic::BasicClient::new(
            oauth2::ClientId::new(CLIENT_ID.to_string()),
            Some(oauth2::ClientSecret::new(CLIENT_SECRET.to_string())),
            oauth2::AuthUrl::new(AUTH_URL.to_string()).expect("Invalid authorization URL"),
            Some(oauth2::TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token URL")),
        )
        .set_revocation_uri(
            oauth2::RevocationUrl::new(REVOCATION_URL.to_string()).expect("Invalid revocation URL"),
        );

        Self {
            oauth_client: client,
            token_data: TokenData {
                access_token: None,
                refresh_token: None,
                expires_at: None,
            },
        }
    }

    pub fn get_token_data(&self) -> TokenData {
        self.token_data.clone()
    }

    pub fn set_token_data(&mut self, token_data: TokenData) {
        self.token_data = token_data;
    }

    pub fn flush(&mut self) {
        self.token_data = TokenData {
            access_token: None,
            refresh_token: None,
            expires_at: None,
        };
    }
}

pub struct PersistedStore {
    store: tauri_plugin_store::Store<tauri::Wry>,
}

impl PersistedStore {
    fn new(app_handle: AppHandle) -> Self {
        let appdata_local = tauri::api::path::app_local_data_dir(&app_handle.config()).unwrap();
        let store_path = appdata_local.join("tokens.json");
        let mut store = tauri_plugin_store::StoreBuilder::new(app_handle, store_path).build();
        store.load().expect("Failed to load store");
        Self { store }
    }

    pub fn save_tokens(&mut self, token_data: &TokenData) {
        println!("Saving tokens: {:?}", token_data);

        if let Some(access_token) = &token_data.access_token {
            self.store
                .insert(
                    "access_token".to_string(),
                    serde_json::Value::String(access_token.clone()),
                )
                .expect("Failed to insert access token");
        }

        if let Some(refresh_token) = &token_data.refresh_token {
            self.store
                .insert(
                    "refresh_token".to_string(),
                    serde_json::Value::String(refresh_token.clone()),
                )
                .expect("Failed to insert refresh token");
        }

        if let Some(expires_at) = &token_data.expires_at {
            self.store
                .insert(
                    "expires_at".to_string(),
                    serde_json::Value::String(expires_at.to_rfc3339()),
                )
                .expect("Failed to insert expires_at");
        }

        self.store.save().expect("Failed to save token store");
    }

    pub fn load_tokens(&mut self) -> TokenData {
        self.store.load().expect("Failed to load token store");

        let access_token = self
            .store
            .get("access_token")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let refresh_token = self
            .store
            .get("refresh_token")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let expires_at = self.store.get("expires_at").and_then(|v| {
            v.as_str().map(|s| {
                DateTime::parse_from_rfc3339(s)
                    .expect("Failed to parse expires_at")
                    .with_timezone(&Utc)
            })
        });

        println!(
            "Loaded tokens: {:?}",
            (&access_token, &refresh_token, &expires_at)
        );

        TokenData {
            access_token,
            refresh_token,
            expires_at,
        }
    }

    pub fn flush(&mut self) {
        self.store.clear().expect("Failed to clear token store");
        self.store.save().expect("Failed to save token store");
    }
}
