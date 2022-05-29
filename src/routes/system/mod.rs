use super::*;

#[put("settings")]
pub async fn put_settings(
    conn: Data<Pool>,
    mut auth: models::Token,
    new: Json<app_state::AppSettings>,
    state: Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let conn = auth_gen::role!(auth, "root", conn.get_conn().await?);

    *state.settings.lock().await = new.0;

    res::no!()
}

#[get("settings")]
pub async fn get_settings(
    conn: Data<Pool>,
    mut auth: models::Token,
    state: Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let conn = auth_gen::role!(auth, "root", conn.get_conn().await?);

    res::json!(*state.settings.lock().await)
}
