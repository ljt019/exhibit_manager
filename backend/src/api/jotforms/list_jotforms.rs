use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Jotform;
use crate::repo::jotform_repo;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

#[get("/jotforms")]
pub async fn list_jotforms_handler(
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<Jotform>>, ApiError> {
    let pool = db_pool.inner().clone();
    let jotforms = jotform_repo::get_all_jotforms(&pool).await?;

    match jotforms {
        Some(jotforms) => Ok(Json(jotforms)),
        None => {
            log::error!("No jotforms found in the database.");
            Err(ApiError::NotFound)
        }
    }
}
