use super::*;
use std::time::Instant;

// #[delete("[{posts}]")]
// pub async fn delete_multiple_json(
//     request: HttpRequest,
//     path: web::Path<(String, String, u64, String)>,
//     pool: Data<Pool>,
// ) -> Result<impl Responder, ApiError> {
//     let conn = pool.get_conn().await?;

//     // find board
//     let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

//     // find thread
//     let (conn, thread) = board.from_route_thread_last(path.2, conn).await?;

//     // check access
//     filter_op_permission(&request, &thread)?;

//     let list = path
//         .3
//         .split(',')
//         .filter_map(|x| {
//             let y = str::parse(x);

//             if let Ok(w) = y {
//                 if w == thread.post_num {
//                     None
//                 } else {
//                     Some(y)
//                 }
//             } else {
//                 Some(y)
//             }
//         })
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

//     if list.len() > 16 || list.is_empty() {
//         return Err(ApiError::new(
//             StatusCode::BAD_REQUEST,
//             Some("VALIDATE"),
//             "TOO_MUCH_POSTS",
//             "Too much posts reported at once (max 16)",
//             None,
//         ));
//     }

//     // find post
//     let (conn, post_ids) = thread.get_all_posts_ids_by_post_nums(&list, conn).await?;
//     let (conn, files) = models::File::related_to_any_post_id(&post_ids, conn).await?;
//     let conn = models::File::delete_related_to_any_post_id(&post_ids, conn).await?;
//     let _ = thread.delete_multiple_posts_by_nums(&list, conn).await?;

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

// #[delete("[{posts}]")]

#[delete("{post:[0-9]+}")]
pub async fn delete_json(
    request: HttpRequest,
    path: web::Path<(String, String, u64, u64)>,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // find post
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

    if !board.op_deletion_enabled {
        Err(ApiError::action_not_allowed_by_board_settings())?
    }

    // find thread
    let (conn, thread) = board.from_route_thread_last(path.2, conn).await?;

    // check access
    filter_op_permission(&request, &thread)?;

    let (mut conn, post) = board.from_route_post_any(path.3, conn).await?;

    let (mut conn, files) = if let Some(id) = post.parent_id {
        if id != thread.id {
            return Err(ApiError::post_invalid_thread());
        }

        let (mut conn, thread) = models::Thread::find(id, conn).await?;
        conn = thread.unwrap().decrement_count(conn).await?;
        conn = post.delete(conn).await?;

        let (conn, files) = models::File::related_to_post_id(post.id, conn).await?;
        (
            models::File::delete_related_to_post_id(post.id, conn).await?,
            files,
        )
    } else {
        if thread.post_id != post.id {
            return Err(ApiError::post_invalid_thread());
        }
        /* if it's a thread !*/
        let (mut conn, mut post_ids) =
            models::Post::find_all_ids_for_thread(thread.id, conn).await?;
        post_ids.extend(&[post.id]);
        conn = models::Thread::delete_by_post_id(post.id, conn).await?;
        let (conn, files) = models::File::related_to_any_post_id(&post_ids, conn).await?;
        (
            models::File::delete_related_to_any_post_id(&post_ids, conn).await?,
            files,
        )
    };
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

pub fn filter_op_permission(
    request: &HttpRequest,
    thread: &models::Thread,
) -> Result<(), ApiError> {
    // get X-Thread-Login header
    let x_thread_login: u64 = request
        .headers()
        .get("x-thread-secret")
        .ok_or(ApiError::thread_secret_not_present())?
        .to_str()
        .map_err(ApiError::from)?
        .parse()
        .map_err(|_| ApiError::token_secret_invalid())?;

    if thread.secret != x_thread_login {
        Err(ApiError::op_secret_invalid())
    } else {
        Ok(())
    }
}

#[post("pin")]
pub async fn pin(
    request: HttpRequest,
    path: web::Path<(String, String, u64, u64)>,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // find board
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

    // find thread
    let (conn, thread) = board.from_route_thread_last(path.2, conn).await?;

    // check access
    filter_op_permission(&request, &thread)?;

    // find post
    let (conn, mut post) = board.from_route_post_any(path.3, conn).await?;

    if let Some(p) = post.parent_id {
        if p != thread.id {
            return Err(ApiError::post_not_related_to_thread());
        }
    } else {
        return Err(ApiError::post_not_related_to_thread());
    }
    let _ = post.pin(conn).await?;

    res::no!()
}

#[post("unpin")]
pub async fn unpin(
    request: HttpRequest,
    path: web::Path<(String, String, u64, u64)>,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    // find board
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

    // find thread
    let (conn, thread) = board.from_route_thread_last(path.2, conn).await?;

    // check access
    filter_op_permission(&request, &thread)?;

    // find post
    let (conn, mut post) = board.from_route_post_any(path.3, conn).await?;

    if let Some(p) = post.parent_id {
        if p != thread.id {
            return Err(ApiError::post_not_related_to_thread());
        }
    } else {
        return Err(ApiError::post_not_related_to_thread());
    }

    let _ = post.unpin(conn).await?;

    res::no!()
}

#[derive(Deserialize)]
pub struct GetQuery {
    pub from: Option<u64>,
    pub full: Option<bool>,
}

// #[get("count")]
// pub async fn count_json(
//     query: Query<GetQuery>,
//     path: web::Path<(String, String, u64)>,
//     pool: Data<Pool>,
// ) -> Result<impl Responder, ApiError> {
//     // get conn
//     let conn = pool.get_conn().await?;

//     let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
//     let (conn, thread) = board.from_route_thread_last(path.2, conn).await?;

//     Ok(response::ok_json(
//         StatusCode::OK,
//         json!(match query.from {
//             Some(v) =>
//                 if v <= thread.count {
//                     thread.count
//                 } else {
//                     let (_conn, v) = thread.get_count_from_num(v, conn).await?;
//                     v
//                 },
//             None => {
//                 thread.count
//             }
//         }),
//     ))
// }

#[get("{page}")]
pub async fn get_page(
    path: web::Path<(String, String, u64, u32)>,
    query: Query<GetQuery>,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    // get conn
    let conn = pool.get_conn().await?;

    // board, thread, oppost
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (conn, (mut thread, mut post)) = board.from_route_thread_post_last(path.2, conn).await?;

    // load files for op-post
    let (conn, _) = post.files(conn).await?;

    // get all replies
    let (_, mut replies) = if thread.endless {
        thread
        .get_replies_with_files_by_page_from_num(
            path.3,
            query.from.unwrap_or(post.num),
            if path.3 == 0 {
                board.bumplimit
            } else {
                board.bumplimit
            },
            conn,
        )
        .await?
    } else {
        thread.get_replies_with_files_from_num(query.from.unwrap_or(post.num), conn).await?
    };

    if let Some(v) = query.from {
        if path.3 == 0 && v <= thread.post_num {
            let mut m = vec![post];
            m.extend(replies);

            replies = m;
        }
    }

    if let Some(true) = query.full {
        thread.posts = Some(replies);
        res::json!(thread)
    } else {
        res::json!(replies)
    }
}

// #[post("")]
pub async fn create_json(
    mparser: web::Data<markup::Parser>,
    path: web::Path<(String, String, u64)>,
    (pool, req): (Data<Pool>, HttpRequest),
    layout: Data<desumark::Layout<config::MarkupErrorType>>,
    watermark: Data<Watermark>,
    multipart: Multipart,
    ip_record: models::Ip,
    mut token: extractors::Extension<Option<models::Token>>,
    app_state: Data<AppState>,
    resolver: Data<TokioDnsResolver>,
) -> Result<impl Responder, ApiError> {
    // get conn
    let conn = pool.get_conn().await?;

    if token.as_ref().is_none()
        && app_state.settings.lock().await.spamhaus_rejection
        && resolver
            .lookup_ip([&ip_record.addr.to_string(), ".zen.spamhaus.org."].concat())
            .await
            .is_ok()
    {
        return Err(ApiError::banned_spamhaus());
    }

    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (mut conn, thread) = board.from_route_thread_last(path.2, conn).await?;

    match req.headers().get("x-thread-secret") {
        Some(header) => {
            if header
                .to_str()
                .map_err(ApiError::from)?
                .parse::<u64>()
                .map_err(|_| ApiError::token_secret_invalid())?
                != thread.secret
            {
                return Err(ApiError::op_secret_invalid());
            }
        }
        None => {}
    }

    if thread.closed == 1 {
        if let Some(ref mut t) = token.as_mut() {
            let perm = [
                "langs::",
                &path.0,
                "::boards::",
                &path.1,
                "::threads::reply_closed",
            ]
            .concat();
            let (nc, status) = t.has_perm(&perm, conn).await?;

            if !status {
                return Err(ApiError::thread_closed());
            }

            conn = nc;
        } else {
            return Err(ApiError::thread_closed());
        }
    }

    let mut state =
        utils::multipart::extract(multipart, watermark, config::MAX_FILES_PER_POST).await?;

    let mut form = match CreateForm::deserialize(Value::Object(state.values.clone())) {
        Ok(v) => v,

        Err(e) => {
            state.cleanup_files().await;

            return Err(e.into());
        }
    };

    // validate form
    match form.validate() {
        Ok(v) => v,

        Err(e) => {
            state.cleanup_files().await;

            return Err(ApiError::validation_error(e));
        }
    };
    
    // tripcode
    let trip = if board.tripcode_enabled {
        if let Some(ref mut name) = form.name {
            let partslen = name.matches('#').count();
            let oldname = name.clone();
            let mut parts = oldname.split("#"); // FIXME
            if partslen > 0 {
                *name = parts.next().unwrap().to_string();
                let rawtrip = parts.collect::<String>();
                let mut hasher = sha2::Sha256::new();
                hasher.update(config::TRIPCODE_SECRET);
                hasher.update(rawtrip);
                Some(format!("{:X}", hasher.finalize()))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // parse
    let mut body: emailoptions::Body = form.into();

    body.apply(None, Some(&board), Some(&thread));

    // create post
    let (mut conn, post) = models::IPost {
        owner: token.as_ref().as_ref().map(|v| v.id),
        ip: ip_record.id,
        board_id: board.id,
        parent_id: Some(thread.id),
        parent_num: Some(thread.post_num),
        pinned: false,
        sage: body.sage,
        op: body.op,
        moder: false,
        trip,
        subject: body.subject.map(|v| v.into()),
        name: body.name.map(|v| v.into()),
        email: body.email.map(|v| v.into()),
        message: mparser.parse(&body.comment, &layout)?,
    }
    .create(conn)
    .await?;

    if !state.files.is_empty() {
        conn = models::InsertMultiple::commit((post.id, &state.files), conn).await?;

        state.untemp_files().await;
    }

    // increment
    thread
        .increment_count(
            !(thread.count >= board.bumplimit as u64 && !thread.endless || body.sage),
            conn,
        )
        .await?;

    app_state
        .set_thread_count_cache((&path.0, &path.1, thread.post_num), thread.count + 1)
        .await;

    app_state
        .last_posts
        .lock()
        .await
        .insert(ip_record.addr, Instant::now());

    // return created post data
    let mut obj = json!(post);

    merge_json(
        &mut obj,
        json!({
            "files": &state.files
        }),
    );

    res::json!(obj)
}
