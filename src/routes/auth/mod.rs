use super::*;

pub mod roles;
pub mod tokens;

#[get("auth")]
pub async fn index_json(
    mut auth: models::Token,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    let (conn, _) = auth.roles(conn).await?;
    let (_conn, _) = auth.perms(conn).await?;

    res::json!(auth)
}
