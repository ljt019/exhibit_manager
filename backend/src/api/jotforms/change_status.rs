use crate::db::DbPool;
use crate::errors::ApiError;
use crate::repo::jotform_repo;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;

#[derive(Debug, Deserialize)]
pub struct ChangeStatusRequest {
    pub new_status: String,
}

#[post("/jotforms/<id>/status", data = "<data>")]
pub async fn change_status_handler(
    db_pool: &State<DbPool>,
    id: i64,
    data: Json<ChangeStatusRequest>,
) -> Result<(), ApiError> {
    let new_status = data.new_status.trim().to_string();
    let pool = db_pool.inner().clone();

    jotform_repo::change_jotform_status(id.to_string(), new_status, &pool).await?;

    Ok(())
}
