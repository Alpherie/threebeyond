use super::*;

pub mod boards;

#[get("langs")]
pub async fn list_json(pool: web::Data<Pool>) -> Result<impl Responder, ApiError> {
    res::json!(models::Lang::all(pool.get_conn().await?).await?.1)
}
