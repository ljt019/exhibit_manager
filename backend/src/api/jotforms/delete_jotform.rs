use crate::db::DbPool;
use crate::errors::ApiError;
use crate::repo::jotform_repo;
use rocket::delete;
use rocket::State;

#[delete("/jotforms/<id>")]
pub async fn delete_jotform_handler(id: i64, db_pool: &State<DbPool>) -> Result<(), ApiError> {
    let pool = db_pool.inner().clone();
    jotform_repo::delete_jotform(id.to_string(), &pool).await?;
    Ok(())
}
