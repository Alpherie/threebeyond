use super::*;

/* create token */
#[derive(Deserialize)]
pub struct CreateData {
    pub roles: Vec<String>,
}

#[post("")]
pub async fn create_json(
    form: web::Json<CreateData>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let conn = auth_gen::role!(auth, "root", conn);

    // create token
    let (mut conn, token) = models::IToken::random(conn).await?;

    // assign roles
    // IDEA: maybe wrap into transaction
    for role in &form.roles {
        let (c, role) = models::Role::any(&role, conn).await?;

        conn = token.assign(&role, c).await?;
    }

    res::json!(token.uuid.to_string())
}

/* delete token */
#[delete("")]
pub async fn delete_json(
    uuid: web::Path<Uuid>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let conn = auth_gen::role!(auth, "root", conn);

    // find token
    let (conn, token) = models::Token::find_by_uuid(&uuid, conn).await?;
    let token = token.ok_or(ApiError::token_not_found(&*uuid))?;

    token.delete(conn).await?.disconnect().await?;

    res::no!()
}

/* add role */
#[derive(Deserialize)]
pub struct AddRoleData {
    pub role: String,
}

#[post("")]
pub async fn add_role_json(
    form: web::Json<AddRoleData>,
    uuid: web::Path<Uuid>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let conn = auth_gen::role!(auth, "root", conn);

    // find token
    let (conn, token) = models::Token::find_by_uuid(&uuid, conn).await?;
    let token = token.ok_or(ApiError::token_not_found(&uuid))?;

    let (conn, role) = models::Role::any(&form.role, conn).await?;
    token.assign(&role, conn).await?;

    res::no!()
}

#[delete("{role}")]
pub async fn delete_role_json(
    path: web::Path<(Uuid, String)>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    let (uuid, role) = path.clone();

    // check access
    let conn = auth_gen::role!(auth, "root", conn);

    // find token
    let (conn, token) = models::Token::find_by_uuid(&uuid, conn).await?;
    let token = token.ok_or(ApiError::token_not_found(&uuid))?;

    let (conn, role) = models::Role::find_by_name(&role, conn).await?;
    let role = role.ok_or(ApiError::role_not_found_name(&path.1))?;

    token.remove_role(&role, conn).await?;

    res::no!()
}
