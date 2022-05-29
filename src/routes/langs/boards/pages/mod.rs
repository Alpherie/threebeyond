use super::*;

#[get("{page}")]
pub async fn get_json(
    path: Path<(String, String, u64)>,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    // get conn
    let conn = pool.get_conn().await?;

    // board
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

    // get page
    let (_conn, page) = board.get_page(path.2, false, conn).await?;

    res::json!(page)
}
