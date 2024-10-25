use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};
use tauri::api::shell;
use tauri::Manager;
use url::Url;

use crate::Tokens;
use chrono::Utc;

fn start_server(tx: std::sync::mpsc::Sender<String>) -> Result<u16, String> {
    let result = tauri_plugin_oauth::start(move |url| {
        if let Err(e) = tx.send(url) {
            println!("[OAuth] Error: Failed to send URL through channel: {}", e);
        }
    });

    match result {
        Ok(port) => {
            println!("[OAuth] Server started on port {}", port);
            Ok(port)
        }
        Err(e) => {
            println!("[OAuth] Error: Failed to start server: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn sign_in(window: tauri::Window) {
    println!("[OAuth] Starting sign-in process");

    // Get existing token store
    let tokens = window.state::<Tokens>();
    let token_store = tokens.0.lock().expect("Failed to lock token store");

    let client = token_store.oauth_client.clone();
    drop(token_store); // Release the lock early

    // Generate PKCE challenge
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Start the server
    let (tx, rx) = std::sync::mpsc::channel();

    let port = start_server(tx).expect("Failed to start server");

    // Set up redirect URL
    let redirect_url = format!("http://localhost:{}", port);
    let client = client.set_redirect_uri(
        oauth2::RedirectUrl::new(redirect_url.clone()).expect("Invalid redirect URL"),
    );

    // Generate authorization URL
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    shell::open(&window.shell_scope(), authorize_url.to_string(), None)
        .expect("Failed to open browser");

    // Wait for redirect
    let redirect_url = match rx.recv() {
        Ok(url) => url,
        Err(e) => {
            println!("[OAuth] Error: Failed to receive redirect URL: {}", e);
            return;
        }
    };

    let url = Url::parse(&redirect_url).expect("Invalid redirect URL");

    // Extract authorization code
    let code_pair = match url.query_pairs().find(|pair| pair.0 == "code") {
        Some(pair) => pair,
        None => {
            println!("[OAuth] Error: Authorization code not found in redirect URL");
            return;
        }
    };
    let code = AuthorizationCode::new(code_pair.1.into_owned());

    let state_pair = match url.query_pairs().find(|pair| pair.0 == "state") {
        Some(pair) => pair,
        None => {
            println!("[OAuth] Error: State not found in redirect URL");
            return;
        }
    };
    let state = CsrfToken::new(state_pair.1.into_owned());

    if state.secret() != csrf_state.secret() {
        println!("[OAuth] Error: CSRF state mismatch");
        return;
    }

    let token_result = match client
        .exchange_code(code)
        .set_pkce_verifier(PkceCodeVerifier::new(
            pkce_code_verifier.secret().to_string(),
        ))
        .request(oauth2::reqwest::http_client)
    {
        Ok(result) => result,
        Err(e) => {
            println!("[OAuth] Error: Failed to exchange code for token: {}", e);
            return;
        }
    };

    let mut token_store = match tokens.0.lock() {
        Ok(store) => store,
        Err(e) => {
            println!(
                "[OAuth] Error: Failed to lock token store for saving: {}",
                e
            );
            return;
        }
    };

    let access_token = token_result.access_token().secret().clone();
    let refresh_token = token_result.refresh_token().map(|t| t.secret().clone());
    let expires_in = token_result.expires_in().expect("No expires in set");

    token_store.access_token = Some(access_token);
    token_store.refresh_token = refresh_token.clone();
    token_store.expires_at = Some(Utc::now() + expires_in);

    token_store.save_tokens(window.app_handle());

    println!("[OAuth] Sign in successful, notifying frontend");

    window
        .emit("sign_in_complete", None::<()>)
        .expect("Failed to emit sign-in event");
}
