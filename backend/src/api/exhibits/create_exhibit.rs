use crate::db::DbPool;
use crate::errors::ApiError;
use crate::repo::exhibit_repo;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

#[derive(serde::Deserialize)]
pub struct NewExhibit {
    pub name: String,
    pub cluster: String,
    pub location: String,
    pub status: String,
    pub image_url: String,
    pub sponsor: Option<crate::models::Sponsor>,
    pub part_ids: Vec<i64>,
    pub notes: Vec<crate::models::Note>,
}

/// Creates a new exhibit with associated parts and notes.
///
/// # Arguments
/// * `new_exhibit` - JSON payload containing the exhibit data
/// * `db_pool` - Database connection pool
///
/// # Returns
/// * `Result<Json<i64>, ApiError>` - The ID of the newly created exhibit
///
/// # Errors
/// Returns `ApiError` if:
/// * Database operations fail
/// * Input validation fails
#[post("/exhibits", format = "json", data = "<new_exhibit>")]
pub async fn create_exhibit_handler(
    new_exhibit: Json<NewExhibit>,
    db_pool: &State<DbPool>,
) -> Result<(), ApiError> {
    let exhibit = new_exhibit.into_inner();
    let pool = db_pool.inner().clone();

    exhibit_repo::create_exhibit(&exhibit, &pool).await?;

    Ok(())
}
