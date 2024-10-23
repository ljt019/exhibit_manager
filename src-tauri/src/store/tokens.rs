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

#[derive(Clone)]
pub struct TokenStore {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            access_token: None,
            refresh_token: None,
            expires_at: None,
        }
    }

    pub fn flush(&mut self) {
        self.access_token = None;
        self.refresh_token = None;
        self.expires_at = None;
    }
}
