mod api;
mod db;
mod errors;
mod jotform;
mod models;

use db::{create_pool, setup_database, DbPool};
use dotenv::dotenv;
use log::{error, info};
use rocket::tokio::time::{sleep, Duration};
use rocket::{catchers, launch, routes};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};
use std::path::Path;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    // Initialize the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting Rocket server...");

    // Check if images directory exists; if not, create it
    if !Path::new("images").exists() {
        std::fs::create_dir("images").expect("Failed to create images directory");
    }

    // Initialize the database connection pool
    let db_pool = create_pool("exhibits.db").expect("Failed to create database connection pool");

    // Setup database schema
    setup_database(&db_pool).expect("Failed to setup database");

    handle_jotform(db_pool.clone());

    // Configure CORS
    let allowed_methods: AllowedMethods = ["Get", "Post", "Delete"]
        .iter()
        .map(|s| std::str::FromStr::from_str(s).unwrap())
        .collect();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&["http://localhost:1420"]))
        .allowed_methods(allowed_methods)
        .allowed_headers(AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
            "Origin",
            "X-Requested-With",
            "Access-Control-Allow-Origin",
        ]))
        .allow_credentials(false)
        .to_cors()
        .expect("Error creating CORS fairing");

    rocket::build()
        .manage(db_pool) // Inject the connection pool into Rocket's state
        .attach(cors) // Attach the CORS fairing
        .mount(
            "/",
            routes![
                api::github::report_bug_handler,
                api::exhibits::create_exhibit_handler,
                api::exhibits::get_exhibit_handler,
                api::exhibits::update_exhibit_handler,
                api::exhibits::delete_exhibit_handler,
                api::exhibits::list_exhibits_handler,
                api::exhibits::handle_random_exhibit,
                api::parts::create_part_handler,
                api::parts::get_part_handler,
                api::parts::update_part_handler,
                api::parts::delete_part_handler,
                api::parts::list_parts_handler,
                api::parts::get_parts_by_ids_handler,
                api::dev::handle_reset_db,
                api::dev::create_dummy_exhibits_handler,
                api::jotforms::list_jotforms_handler,
            ],
        )
        .mount("/images", rocket::fs::FileServer::from("images"))
        .register(
            "/",
            catchers![
                errors::not_found,
                errors::handle_invalid_request_body,
                errors::handle_method_not_allowed,
                errors::internal_server_error
            ],
        )
}

/// Fetches data from Jotform and stores it in the database
///
/// makes sure not to store duplicate data, and updates existing data
/// if new data is available from Jotform API.
pub fn handle_jotform(db_pool: DbPool) {
    let jotform_api_key = std::env::var("JOTFORM_API_KEY").expect("JOTFORM_API_KEY not set");
    let jotform_form_id = std::env::var("JOTFORM_FORM_ID").expect("JOTFORM_FORM_ID not set");
    let jotform_base_url = "https://api.jotform.com".to_string();

    let jotform_api_client =
        jotform::JotformApi::new(jotform_api_key, jotform_form_id, jotform_base_url);

    let pool_clone = db_pool.clone();
    let api_clone = jotform_api_client;

    rocket::tokio::spawn(async move {
        loop {
            match jotform::sync_jotforms_once(&pool_clone, &api_clone).await {
                Ok(_) => info!("Successfully synced Jotform data"),
                Err(e) => error!("Failed to sync Jotform data: {:?}", e),
            }

            // Sleep for 12 hours (so refresh twice a day)
            sleep(Duration::from_secs(43200)).await;
        }
    });
}
