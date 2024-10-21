use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};
use tauri::api::shell;
use tauri::Manager;
use url::Url;

use std::sync::mpsc;

use super::auth::get_client;
use crate::Tokens;

#[tauri::command]
pub fn sign_in(window: tauri::Window) {
    let (tx, rx) = mpsc::channel();

    let client = get_client(window.clone(), tx).expect("Failed to get client");

    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    // Open the authorization URL in the user's default browser.
    shell::open(&window.shell_scope(), authorize_url.to_string(), None)
        .expect("Failed to open browser");

    // Wait for the redirect URL containing the authorization code.
    let redirect_url = rx.recv().expect("Failed to receive redirect URL");
    let url = Url::parse(&redirect_url).expect("Invalid redirect URL");

    // Extract the authorization code and state from the redirect URL.
    let code_pair = url
        .query_pairs()
        .find(|pair| pair.0 == "code")
        .expect("Authorization code not found");
    let code = AuthorizationCode::new(code_pair.1.into_owned());

    let state_pair = url
        .query_pairs()
        .find(|pair| pair.0 == "state")
        .expect("State not found");
    let state = CsrfToken::new(state_pair.1.into_owned());

    // Verify the CSRF state.
    if state.secret() != csrf_state.secret() {
        panic!("CSRF state mismatch");
    }

    // Exchange the authorization code for an access token.
    let token_result = client
        .exchange_code(code)
        .set_pkce_verifier(PkceCodeVerifier::new(
            pkce_code_verifier.secret().to_string(),
        ))
        .request(oauth2::reqwest::http_client)
        .expect("Failed to exchange code for token");

    println!("Access token: {:?}", token_result.access_token().secret());

    // Store the access token securely
    // Just storing in memory for now, eventually should be persisted securely
    let access_token = token_result.access_token().secret().clone();

    {
        // Store the state in a variable to extend its lifetime
        let tokens_state = window.state::<Tokens>();
        let mut tokens = tokens_state.0.lock().expect("Failed to lock tokens");
        tokens.access_token = Some(access_token.clone());
    }

    // Emit event to let the frontend know that the user is signed in
    window
        .emit("sign_in_complete", None::<()>)
        .expect("Failed to emit signed-in event");
}
