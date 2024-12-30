use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Jotform;
use crate::repo::jotform_repo;
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

#[get("/jotforms/<id>")]
pub async fn get_jotform_handler(
    id: String,
    db_pool: &State<DbPool>,
) -> Result<Json<Jotform>, ApiError> {
    let pool = db_pool.inner().clone();

    let jotform = jotform_repo::get_jotform(id.to_string(), &pool).await?;

    match jotform {
        Some(jotform) => Ok(Json(jotform)),
        None => {
            error!("Jotform not found");
            Err(ApiError::NotFound)
        }
    }
}
