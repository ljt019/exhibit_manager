use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl};
use tauri_plugin_oauth::start;

use std::sync::mpsc;

pub fn get_client(window: tauri::Window, tx: mpsc::Sender<String>) -> Result<BasicClient, String> {
    // Replace with your client ID and client secret.
    const CLIENT_ID: &str =
        "883531815886-cr4nl5k7v4hf81bhmfjhe39j3r2ic5lm.apps.googleusercontent.com";
    const CLIENT_SECRET: &str = "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";
    const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
    const TOKEN_URL: &str = "https://accounts.google.com/o/oauth2/token";
    const REVOCATION_URL: &str = "https://oauth2.googleapis.com/revoke";

    let port = start_server(window, tx).expect("Failed to start server");

    println!("Server started at port: {}", port);

    let redirect_url = format!("http://localhost:{}", port);

    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
        AuthUrl::new(AUTH_URL.to_string()).expect("Invalid authorization URL"),
        Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token URL")),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URL"))
    .set_revocation_uri(
        RevocationUrl::new(REVOCATION_URL.to_string()).expect("Invalid revocation URL"),
    );

    Ok(client)
}

pub fn start_server(window: tauri::Window, tx: mpsc::Sender<String>) -> Result<u16, String> {
    start(move |url| {
        // Verify the URL and send it back via the channel.
        let _ = window.emit("redirect_uri", url.clone());
        tx.send(url).unwrap();
    })
    .map_err(|err| err.to_string())
}
