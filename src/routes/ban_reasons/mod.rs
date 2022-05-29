use super::*;

//--
#[derive(Deserialize)]
pub struct BanReasonCreateForm {
    pub alias: String,
}

#[post("")]
pub async fn create(
    j: web::Json<BanReasonCreateForm>,
    mut auth: models::Token,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let perm = "reasons::create";
    let conn = auth_gen::perm!(auth, &perm, conn);

    let (conn, q) = models::BanReason::find_by_alias(&j.alias, conn).await?;
    if let Some(_) = q {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            Some("CREATE"),
            "REASON_ALREADY_EXISTS",
            "Reason is alreayd exists",
            None,
        ));
    }

    models::IBanReason { alias: &j.alias }.create(conn).await?;

    res::no!()
}

//---
#[get("reasons")]
pub async fn list(pool: Data<Pool>) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    res::json!(models::BanReason::all(conn)
        .await?
        .1
        .into_iter()
        .map(|v| v.alias)
        .collect::<Vec<_>>())
}
