use super::*;
use std::time::Instant;
use sha2::{Sha256, Digest};

pub mod posts;

#[derive(Deserialize, Validate)]
pub struct CreateForm {
    #[validate(length(max = 128))]
    pub subject: Option<String>,

    #[validate(length(max = 128))]
    pub email: Option<String>,

    #[validate(length(max = 128))]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 50000))]
    #[serde(alias = "comment")]
    pub message: String,
}

impl Into<emailoptions::Body> for CreateForm {
    fn into(self) -> emailoptions::Body {
        emailoptions::Body {
            subject: self
                .subject
                .as_ref()
                .map(|v| htmlescape::encode_minimal(v).into_boxed_str()),
            email: self
                .email
                .as_ref()
                .map(|v| htmlescape::encode_minimal(v).into_boxed_str()),
            name: self
                .name
                .as_ref()
                .map(|v| htmlescape::encode_minimal(v).into_boxed_str()),
            comment: htmlescape::encode_minimal(&self.message).into_boxed_str(),
            op: false,
            sage: false,
            good: 0,
            bad: 0,
        }
    }
}

// #[post("")]
pub async fn create_json(
    mparser: web::Data<markup::Parser>,
    path: web::Path<(String, String)>,
    pool: Data<Pool>,
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

    // get board
    let (mut conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

    if token.as_ref().is_none()
        && app_state.settings.lock().await.spamhaus_rejection
        && resolver
            .lookup_ip([&ip_record.addr.to_string(), ".zen.spamhaus.org."].concat())
            .await
            .is_err()
    {
        Err(ApiError::banned_spamhaus())?
    }

    if let Some(seconds) = board.slowmode {
        let border = board.last_thread_created + Duration::seconds(seconds as i64);
        let now = time_now();
        if border > now {
            Err(ApiError::board_slowmode_remaining(
                (border - now).num_seconds(),
            ))?;
        }
    }

    if board.thread_creating_limited == 1 {
        let perm = [
            "langs::",
            &path.0,
            "::boards::",
            &path.1,
            "::posting::create_thread",
        ]
        .concat();

        match token.as_mut() {
            Some(t) => {
                conn = auth_gen::perm!(t, &perm, conn);
            }

            None => return Err(ApiError::access_missing_perm(&perm)),
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

    body.apply(None, Some(&board), None);
 
    // create post
    let (conn, post) = models::IPost {
        owner: token.as_ref().as_ref().map(|v| v.id),
        ip: ip_record.id,
        board_id: board.id,
        parent_id: None,
        parent_num: None,
        pinned: false,
        sage: body.sage,
        op: true,
        moder: false,
        subject: body.subject.map(|v| v.into()),
        trip,
        name: body.name.map(|v| v.into()),
        email: body.email.map(|v| v.into()),
        message: mparser.parse(&body.comment, &layout)?,
    }
    .create(conn)
    .await?;

    // create thread
    let (mut conn, thread) = models::IThread {
        owner: None,
        ip: ip_record.id,
        board_id: board.id,
        post_id: post.id,
        post_num: post.num,
        sticky: 0,
        endless: false,
        secret: rand::thread_rng().gen_range(0, std::u64::MAX),
    }
    .create(conn)
    .await?;

    conn = board.update_last_thread(conn).await?;

    if !state.files.is_empty() {
        conn = models::InsertMultiple::commit((post.id, &state.files), conn).await?;

        state.untemp_files().await;
    }

    // return created post data
    app_state
        .last_posts
        .lock()
        .await
        .insert(ip_record.addr, Instant::now());
    let mut post = json!(post);

    merge_json(
        &mut post,
        json!({
            "files": &state.files
        }),
    );

    // return created post data
    res::json!(json!({"post": post, "secret": thread.secret}))
}

#[get("{num}")]
pub async fn get_json(
    path: web::Path<(String, String, u64)>,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    // get conn
    let conn = pool.get_conn().await?;

    // board, thread, oppost
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (conn, (thread, mut post)) = board.from_route_thread_post_last(path.2, conn).await?;
    let (conn, _) = post.files(conn).await?;

    // get all replies
    let (_conn, replies) = if thread.endless {
        thread
            .get_replies_with_files_by_page(0, board.bumplimit, conn)
            .await?
    } else {
        thread
            .get_replies_with_files_from_num(post.num - 1, conn)
            .await?
    };
    let replies = {
        let mut v = vec![post];
        v.extend(replies);
        v
    };

    let mut j = json!(thread);
    crate::merge_json(&mut j, json!({ "posts": replies }));

    // // add pinned property
    // let pinned: Vec<u64> = replies
    //     .iter()
    //     .filter_map(|p| if p.pinned { Some(p.num) } else { None })
    //     .collect();
    // crate::merge_json(&mut j, json!({ "pinned": pinned }));

    res::json!(j)
}
// catalog
#[get("threads")]
pub async fn catalog_json(
    path: web::Path<(String, String)>,
    pool: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    // get conn
    let conn = pool.get_conn().await?;

    // get board
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;

    // get all alive threads and post for each
    let (conn, mut t): (_, Vec<models::Thread>) =
        models::Thread::last_threads_on_board_with_post_as_reply(
            board.id,
            board.pages_count * board.per_page,
            conn,
        )
        .await?;

    let (conn, files) = models::File::all_for_posts(t.iter().map(|t| t.post_id), conn).await?;

    let mut post_postnum_map = t
        .iter_mut()
        .map(|t| (t.post_id, t.posts.as_mut().unwrap().get_mut(0)))
        .collect::<HashMap<_, _>>();

    for file in files {
        let target = post_postnum_map
            .get_mut(&file.post)
            .unwrap()
            .as_mut()
            .unwrap();

        match target.files.as_mut() {
            Some(v) => v.push(file),
            None => target.files = Some(vec![file]),
        };
    }

    res::json!(t)
}

#[post("close")]
pub async fn close(
    path: Path<(String, String, u64)>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;
    let perm = [
        "langs::",
        &path.0,
        "::boards::",
        &path.1,
        "::threads::close",
    ]
    .concat();
    let conn = auth_gen::perm!(auth, &perm, conn);
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (mut conn, mut thread) = board.from_route_thread_last(path.2, conn).await?;
    conn = thread.close(conn).await?;
    res::no!()
}

#[post("open")]
pub async fn open(
    path: Path<(String, String, u64)>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;
    let perm = ["langs::", &path.0, "::boards::", &path.1, "::threads::open"].concat();
    let conn = auth_gen::perm!(auth, &perm, conn);
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (mut conn, mut thread) = board.from_route_thread_last(path.2, conn).await?;
    conn = thread.open(conn).await?;
    res::no!()
}

#[post("pin")]
pub async fn pin(
    path: Path<(String, String, u64)>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;
    let perm = ["langs::", &path.0, "::boards::", &path.1, "::threads::pin"].concat();
    let conn = auth_gen::perm!(auth, &perm, conn);
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (mut conn, mut thread) = board.from_route_thread_last(path.2, conn).await?;
    conn = thread.pin(conn).await?;
    res::no!()
}

#[post("unpin")]
pub async fn unpin(
    path: Path<(String, String, u64)>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;
    let perm = [
        "langs::",
        &path.0,
        "::boards::",
        &path.1,
        "::threads::unpin",
    ]
    .concat();
    let conn = auth_gen::perm!(auth, &perm, conn);
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (mut conn, mut thread) = board.from_route_thread_last(path.2, conn).await?;
    conn = thread.unpin(conn).await?;
    res::no!()
}

#[post("endless")]
pub async fn endless(
    path: Path<(String, String, u64)>,
    pool: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;
    let perm = [
        "langs::",
        &path.0,
        "::boards::",
        &path.1,
        "::threads::make_endless",
    ]
    .concat();
    let conn = auth_gen::perm!(auth, &perm, conn);
    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let (mut conn, mut thread) = board.from_route_thread_last(path.2, conn).await?;
    conn = thread.make_endless(conn).await?;
    res::no!()
}
