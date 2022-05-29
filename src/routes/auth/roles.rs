use super::*;

// remove perm
#[delete("")]
pub async fn remove_perm(
    path: web::Path<(String, String)>,
    pool: web::Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    let conn = auth_gen::role!(auth, "root", conn);

    let (conn, role) = models::Role::find_by_name(&path.0, conn).await?;
    let mut role = role.ok_or(ApiError::role_not_found(&path.0))?;

    let (conn, perm) = models::Perm::find_by_name(&path.1, conn).await?;
    let perm = perm.ok_or(ApiError::perm_not_found_name(&path.1))?;

    role.remove_perm(&perm, conn).await?;

    res::no!()
}

/* get info */
#[get("{role}")]
pub async fn info(
    role: web::Path<String>,
    pool: web::Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let conn = auth_gen::role!(auth, "root", conn);

    let (conn, orole) = models::Role::find_by_name(&role, conn).await?;
    let role = orole.ok_or(ApiError::role_not_found(role.as_ref()))?;

    let (_conn, perms) = models::Perm::related_to_role(role.id, conn).await?;

    let perms = perms.iter().map(|v| &v.name).collect::<Box<[_]>>();

    res::json!(json!({ "role": role.name, "perms": perms }))
}

/* perms */
#[derive(Deserialize)]
pub struct AddPermData {
    pub name: String,
}

#[post("")]
pub async fn add_perm_json(
    role: web::Path<String>,
    form: web::Json<AddPermData>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let conn = auth_gen::role!(auth, "root", conn);

    let (conn, orole) = models::Role::find_by_name(&role, conn).await?;
    let role = orole.ok_or(ApiError::role_not_found(role.as_ref()))?;

    let (conn, perm) = models::Perm::any(&form.name, conn).await?;
    role.assign(&perm, conn).await?;

    res::no!()
}

/* create role */
#[derive(Deserialize)]
pub struct CreateData {
    pub name: String,
    pub perms: Vec<String>,
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
    let (mut conn, role) = models::IRole { name: &form.name }
        .create(conn)
        .await
        .map_err(|_| ApiError::cannot_create_role_duplicate(&form.name))?;

    // assign roles
    // IDEA: maybe wrap into transaction
    for perm in &form.perms {
        let (c, perm) = models::Perm::any(&perm, conn).await?;

        conn = role.assign(&perm, c).await?;
    }

    res::json!(role)
}
