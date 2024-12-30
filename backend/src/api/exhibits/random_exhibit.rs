use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Exhibit;
use crate::repo::exhibit_repo;
use rand::seq::SliceRandom;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

#[get("/exhibits/random")]
pub async fn handle_random_exhibit(db_pool: &State<DbPool>) -> Result<Json<Exhibit>, ApiError> {
    let pool = db_pool.inner().clone();
    let exhibits = exhibit_repo::get_all_exhibits(&pool).await?;

    match exhibits {
        Some(exhibits) => {
            let random_exhibit = exhibits
                .choose(&mut rand::thread_rng())
                .ok_or(ApiError::NotFound)?
                .clone();
            Ok(Json(random_exhibit))
        }
        None => Err(ApiError::NotFound),
    }
}
