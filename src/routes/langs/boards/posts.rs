use super::*;
use std::borrow::Cow;

#[get("{post:[0-9]+}")]
pub async fn get_json(
    path: web::Path<(String, String, u64)>,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // find post
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (conn, mut post) = board.from_route_post_any(path.2, conn).await?;
    let _ = post.files(conn).await?;

    res::json!(post)
}

#[delete("{post:[0-9]+}")]
pub async fn delete_json(
    path: web::Path<(String, String, u64)>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // check access
    let perm = ["langs::", &path.0, "::boards::", &path.1, "::posts::delete"].concat();
    let conn = auth_gen::perm!(auth, &perm, conn);

    // find post
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (conn, post) = board.from_route_post_any(path.2, conn).await?;

    let (mut conn, files) = if post.parent_id == None {
        /* if it's a thread !*/
        let (conn, thread) = models::Thread::find_by_post_id(post.id, conn).await?;
        let thread = thread.unwrap();
        let (mut conn, mut post_ids) =
            models::Post::find_all_ids_for_thread(thread.id, conn).await?;
        post_ids.extend(&[post.id]);
        conn = models::Thread::delete_by_post_id(post.id, conn).await?;
        conn = models::Post::delete_by_parent_id(thread.id, conn).await?;
        let (conn, files) = models::File::related_to_any_post_id(&post_ids, conn).await?;
        (
            models::File::delete_related_to_any_post_id(&post_ids, conn).await?,
            files,
        )
    } else {
        let (conn, files) = models::File::related_to_post_id(post.id, conn).await?;
        (
            models::File::delete_related_to_post_id(post.id, conn).await?,
            files,
        )
    };

    conn = post.delete(conn).await?;
    if let Some(id) = post.parent_id {
        let (conn, thread) = models::Thread::find(id, conn).await?;

        let _ = thread.unwrap().decrement_count(conn).await?;
    }
    for file in files {
        let _ = tokio::fs::remove_file(format!(
            "uploads/full/{}.{}",
            file.uuid,
            file.extension.as_str()
        ))
        .await;
        let _ = tokio::fs::remove_file(format!("uploads/thumbs/{}.png", file.uuid)).await;
    }

    res::no!()
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum BanScope {
    Board,
    Lang,
    Service,
}

#[derive(Validate, Deserialize)]
pub struct BanForm {
    pub scope: BanScope,
    pub reason: String,
    #[validate(length(min = 2, max = 127))]
    pub comment: Option<String>,
    pub minutes: u32,
    pub subnet: Option<bool>,
}

#[post("ban")]
pub async fn ban_json(
    path: web::Path<(String, String, u64)>,
    pool: Data<Pool>,
    form: web::Json<BanForm>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    form.validate()?;

    let conn = pool.get_conn().await?;

    // find the post
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (conn, post) = board.from_route_post_any(path.2, conn).await?;

    let (perm, lang, board): (_, Option<&str>, Option<u64>) = match form.scope {
        BanScope::Board => (
            Cow::Owned(
                [
                    "langs::",
                    &path.0,
                    "::boards::",
                    &path.1,
                    if let Some(true) = form.subnet {
                        "::posts::ban_subnet"
                    } else {
                        "::posts::ban"
                    },
                ]
                .concat(),
            ),
            Some(&board.lang),
            Some(board.id),
        ),

        BanScope::Lang => (
            Cow::Owned(
                [
                    "langs::",
                    &path.0,
                    if let Some(true) = form.subnet {
                        "::ban_subnet"
                    } else {
                        "::ban"
                    },
                ]
                .concat(),
            ),
            Some(&board.lang),
            None,
        ),

        BanScope::Service => (
            Cow::Borrowed(if let Some(true) = form.subnet {
                "::ban_subnet"
            } else {
                "::ban"
            }),
            None,
            None,
        ),
    };

    // check access
    let conn = auth_gen::perm!(auth, &perm, conn);

    // find reason
    let (conn, reason) = models::BanReason::from_route_by_alias(&form.reason, conn).await?;

    match post.owner {
        Some(token_id) => {
            models::ITokenBan {
                token: token_id,
                lang,
                board,
                comment: form.comment.as_ref().map(|v| v.as_str()).unwrap_or(""),
                reason: reason.id,
                minutes: form.minutes,
                because: None, // FIXME: add support
            }
            .create(conn)
            .await?;
        }

        None => {
            // get ip of the poster
            let (conn, ip) = models::Ip::find(post.ip, conn).await?;
            let ip = ip.unwrap(); // :/

            let mut ip: models::BanIpAddress = ip.addr.into();

            // FIXME: lol, those functional patterns :D, just replace
            if let Some(true) = form.subnet {
                ip = match ip {
                    models::BanIpAddress::V4(subnet, _) => models::BanIpAddress::V4(subnet, None),
                    models::BanIpAddress::V6(subnet, _) => models::BanIpAddress::V6(subnet, None),
                }
            }

            let (_, _ban) = models::IBan {
                lang,
                board,
                comment: form.comment.as_ref().map(|v| v.as_str()).unwrap_or(""),
                reason: reason.id,
                ip,
                minutes: form.minutes,
                because: None, // FIXME: add support
            }
            .create(conn)
            .await?;
        }
    }
    res::no!()
}

#[derive(Deserialize, Validate)]
pub struct ReportForm {
    #[validate(length(min = 2, max = 64))]
    pub comment: String,
}

#[post("report")]
pub async fn report_multiple_json(
    req: HttpRequest,
    path: web::Path<(String, String, String)>,
    pool: Data<Pool>,
    form: web::Json<ReportForm>,
    ip_record: models::Ip,
    token: extractors::Extension<Option<models::Token>>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;
    form.validate()?;

    let list = path
        .2
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|_| {
            ApiError::new(
                StatusCode::BAD_REQUEST,
                Some("VALIDATE"),
                "POST_NUM_INVALID",
                "Post num is invalid",
                None,
            )
        })?;

    if list.len() > 16 || list.is_empty() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            Some("VALIDATE"),
            "TOO_MUCH_POSTS",
            "Too much posts reported at once (max 16)",
            None,
        ));
    }

    // check if user already reported MAX amount
    let (conn, amount) = match token.as_ref() {
        Some(v) => models::Report::count_amount_last_hour_by_token_id(v.id, conn).await?,
        None => models::Report::count_amount_last_hour_by_ip(ip_record.id, conn).await?,
    };

    if amount > config::MAX_REPORTS {
        return Err(ApiError::max_reports_amount_sent());
    }

    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

    // find all posts by num on board
    let (conn, posts) = models::Post::find_all_where_num_in_on_board(&list, board.id, conn).await?;
    let fp = &posts[0];
    let (thread_id, thread_num) = (
        fp.parent_id.unwrap_or(fp.id),
        fp.parent_num.unwrap_or(fp.num),
    );

    for post in posts {
        if post.parent_id.is_some() && post.parent_id != Some(thread_id) {
            return Err(ApiError::report_select_not_same_thread());
        }
    }

    // some magic to prevent proxy checking :DDDDD
    let (conn, ignored) = match token.as_ref().as_ref() {
        Some(v) => {
            let (conn, ban) = models::TokenBan::find_by_token_id(v.id, conn).await?;
            (conn, ban.is_some() as u8)
        }

        None => {
            let ip = req.peer_addr().unwrap().ip();
            let prep: models::BanIpAddress = ip.into();
            let (conn, ban) =
                models::Ban::find_wn_lang_wn_board_first(prep, &path.0, &path.1, conn).await?;
            (conn, ban.is_some() as u8)
        }
    };

    let (mut conn, report) = models::IReport {
        ignored,
        ip: ip_record.id,
        owner: token.as_ref().as_ref().map(|v| v.id),
        lang: &path.0,
        board_id: board.id,
        thread_id,
        thread_num,
        board_short: &path.1,
        comment: &form.comment,
    }
    .create(conn)
    .await?;

    // FIXME: transcation
    for num in list {
        conn = report.add_post(num, conn).await?;
    }

    res::no!()
}

// #[delete("[{posts}]")]
// pub async fn delete_multiple_json(
//     path: web::Path<(String, String, String)>,
//     pool: Data<Pool>,
//     mut auth: models::Token,
// ) -> Result<impl Responder, ApiError> {
//     let conn = pool.get_conn().await?;

//     let list = path
//         .2
//         .split(',')
//         .map(str::parse)
//         .collect::<Result<Vec<u64>, _>>()
//         .map_err(|_| {
//             ApiError::new(
//                 StatusCode::BAD_REQUEST,
//                 Some("VALIDATE"),
//                 "POST_NUMS_INVALID",
//                 "Post nums are invalid",
//                 None,
//             )
//         })?;

//     // check access
//     let perm = ["langs::", &path.0, "::boards::", &path.1, "::posts::delete"].concat();

//     let conn = auth_gen::perm!(auth, &perm, conn);

//     // find post
//     let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
//     let (conn, post_ids) = board.get_all_posts_ids_by_post_nums(&list, conn).await?;
//     let (conn, files) = models::File::related_to_any_post_id(&post_ids, conn).await?;
//     let conn = models::File::delete_related_to_any_post_id(&post_ids, conn).await?;
//     let _ = board.delete_multiple_posts_by_nums(&list, conn).await?;

//     for file in files {
//         let _ = tokio::fs::remove_file(format!(
//             "uploads/full/{}.{}",
//             file.uuid,
//             file.extension.as_str()
//         ))
//         .await;
//         let _ = tokio::fs::remove_file(format!("uploads/thumbs/{}.png", file.uuid)).await;
//     }

//     res::no!()
// }

#[get("details")]
pub async fn details(
    mut auth: models::Token,
    path: web::Path<(String, String, u64)>,
    pool: web::Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;
    let perm = [
        "langs::",
        &path.0,
        "::boards::",
        &path.1,
        "::posts::view_details",
    ]
    .concat();
    let conn = auth_gen::perm!(auth, &perm, conn);

    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (conn, post) = board.from_route_post_any(path.2, conn).await?;
    let (_conn, ip) = models::Ip::find(post.ip, conn).await?;
    let ip = ip.unwrap().addr.to_string();

    res::json!(json!({
        "ip": ip,
        "token": post.owner
    }))
}
