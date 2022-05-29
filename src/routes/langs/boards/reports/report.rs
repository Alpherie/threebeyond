use super::*;

#[post("solve")]
pub async fn solve(
    path: web::Path<(String, String, u64)>,
    mut auth: models::Token,
    pool: web::Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let perm = [
        "langs::",
        &path.0,
        "::boards::",
        &path.1,
        "::reports::solve",
    ]
    .concat();
    let conn = auth_gen::perm!(auth, &perm, conn);

    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

    let (conn, report) = models::Report::find_on_board_id(path.2, board.id, conn).await?;
    let mut report = match report {
        Some(v) => v,
        None => return Err(ApiError::report_not_found()),
    };

    if report.is_solved() {
        return Err(ApiError::report_already_solved());
    }

    report.solve(auth.id, conn).await?;

    res::no!()
}
