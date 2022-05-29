use super::*;

#[derive(Deserialize)]
pub struct PageQuery {
    pub unsolved: Option<bool>,
}

#[get("{page}")]
pub async fn page(
    mut auth: models::Token,
    query: web::Query<PageQuery>,
    path: web::Path<(String, String, u64)>,
    pool: web::Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let perm = ["langs::", &path.0, "::boards::", &path.1, "::reports::list"].concat();
    let conn = auth_gen::perm!(auth, &perm, conn);

    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (conn, mut reports) = match query.unsolved {
        Some(true) => board.get_reports_page_unsolved(path.2, conn).await?,
        _ => board.get_reports_page(path.2, conn).await?,
    };

    let report_ids = reports.iter().map(|r| r.id).collect::<Vec<_>>();

    let (_conn, nums) = models::Report::get_raw_nums_for_reports(&report_ids, conn).await?;

    let mut idx = 0;
    for num in nums {
        let report_id = num.0;

        if reports[idx].id != report_id {
            while reports[idx].id != report_id {
                idx += 1;
            }
        }

        reports[idx].post_nums.push(num.1);
    }

    res::json!(reports)
}
