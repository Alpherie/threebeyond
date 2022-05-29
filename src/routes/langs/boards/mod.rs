use super::*;

pub mod pages;
pub mod posts;
pub mod reports;
pub mod threads;

lazy_static! {
    static ref SHORT_FORMAT: Regex = Regex::new(r"[a-zA-Z0-9]+").unwrap();
}

#[derive(Validate, Deserialize)]
pub struct CreateBoardData {
    #[validate(length(min = 1, max = 16), regex = "SHORT_FORMAT")]
    pub short: String,
    pub name: String,
    pub description: String,
    pub rules: String,
    pub pages_count: u64,
    pub per_page: u64,
    pub last_replies: u64,
    pub bumplimit: u32,
    pub thread_creating_limited: bool,
    pub slowmode: Option<u64>,
    pub op_oppost_enabled: bool,
    pub op_deletion_enabled: bool,
    pub tripcode_enabled: bool
}

#[post("")]
pub async fn create_json(
    lang_code: Path<String>,
    j: Json<CreateBoardData>,
    p: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    j.validate()?;
    let conn = p.get_conn().await?;

    let perm = ["langs::", &lang_code, "::boards::create"].concat();
    let conn = auth_gen::perm!(&mut auth, &perm, conn);

    let (conn, lang) = models::Lang::find_by_code(&lang_code, conn).await?;
    if lang.is_none() {
        Err(ApiError::lang_not_found(lang_code.as_ref()))?;
    }

    let (conn, board) = models::Board::find_by_short_for_lang(&j.short, &lang_code, conn).await?;

    if board.is_some() {
        Err(ApiError::board_already_exists(lang_code.as_ref(), &j.short))?;
    }

    let (conn, board) = models::IBoard {
        lang: &lang_code,
        short: &j.short,
        name: &j.name,
        description: &j.description,
        rules: &j.rules,
        pages_count: j.pages_count,
        per_page: j.per_page,
        last_replies: j.last_replies,
        bumplimit: j.bumplimit,
        thread_creating_limited: if j.thread_creating_limited { 1 } else { 0 },
        slowmode: j.slowmode,
        op_oppost_enabled: j.op_oppost_enabled,
        op_deletion_enabled: j.op_deletion_enabled,
        last_thread_created: time_now(),
        tripcode_enabled: j.tripcode_enabled
    }
    .create(conn)
    .await?;

    let base = ["langs::", &lang_code, "::boards::", &j.short, "::"].concat();
    let (conn, moder) = models::Role::any(&[&base, "moder"].concat(), conn).await?;
    let (mut conn, admin) = models::Role::any(&[&base, "admin"].concat(), conn).await?;

    for perm in perm_rep::general_board_perms {
        let perm_dir = [&base, *perm].concat();
        let (nc, perm) = models::Perm::any(&perm_dir, conn).await?;
        conn = moder.assign(&perm, nc).await?;
    }

    for perm in perm_rep::admin_board_perms {
        let perm_dir = [&base, *perm].concat();
        let (nc, perm) = models::Perm::any(&perm_dir, conn).await?;
        conn = admin.assign(&perm, nc).await?;
    }

    conn = auth.assign(&moder, conn).await?;
    conn = auth.assign(&admin, conn).await?;

    res::json!(board)
}

#[derive(Validate, Deserialize)]
pub struct UpdateBoardData {
    pub name: String,
    pub description: String,
    pub rules: String,
    pub pages_count: u64,
    pub per_page: u64,
    pub last_replies: u64,
    pub bumplimit: u32,
    pub thread_creating_limited: bool,
    pub slowmode: Option<u64>,
    pub op_oppost_enabled: bool,
    pub op_deletion_enabled: bool,
    pub tripcode_enabled: bool
}

#[put("{board}")]
pub async fn update_json(
    path: Path<(String, String)>,
    j: Json<UpdateBoardData>,
    p: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = p.get_conn().await?;
    let j = j.0;

    let perm = ["langs::", &path.0, "::boards::", &path.1, "::control"].concat();
    let conn = auth_gen::perm!(&mut auth, &perm, conn);

    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let board = models::Board {
        name: j.name,
        description: j.description,
        rules: j.rules,
        pages_count: j.pages_count,
        per_page: j.per_page,
        last_replies: j.last_replies,
        bumplimit: j.bumplimit,
        thread_creating_limited: if j.thread_creating_limited { 1 } else { 0 },
        slowmode: j.slowmode,
        op_oppost_enabled: j.op_oppost_enabled,
        op_deletion_enabled: j.op_deletion_enabled,
        tripcode_enabled: j.tripcode_enabled,
        ..board
    };

    let _ = board.save_all(conn).await?;

    res::no!()
}

#[delete("{board}")]
pub async fn delete_json(
    path: Path<(String, String)>,
    p: Data<Pool>,
    mut auth: models::Token,
) -> Result<impl Responder, ApiError> {
    let conn = p.get_conn().await?;

    let perm = ["langs::", &path.0, "::boards::", &path.1, "::control"].concat();
    let conn = auth_gen::perm!(&mut auth, &perm, conn);

    let (conn, board) = models::Board::from_route_lang_short(&path.0, &path.1, conn).await?;
    let conn = board.delete(conn).await?;

    let base = ["langs::", &path.0, "::boards::", &path.1, "::"].concat();
    let (conn, moder) = models::Role::any(&[&base, "moder"].concat(), conn).await?;
    let (conn, admin) = models::Role::any(&[&base, "admin"].concat(), conn).await?;

    let conn = auth.remove_role(&moder, conn).await?;
    let _ = auth.remove_role(&admin, conn).await?;

    res::no!()
}

#[get("boards")]
pub async fn list_json(lang: Path<String>, p: Data<Pool>) -> Result<impl Responder, ApiError> {
    let (_, boards) = models::Board::all_for_lang(&lang, p.get_conn().await?).await?;

    if boards.is_empty() {
        Err(ApiError::new(
            StatusCode::NOT_FOUND,
            None::<&str>,
            "LANG_NOT_SUPPORTED",
            "This language is not supported",
            None,
        ))
    } else {
        res::json!(boards)
    }
}

#[get("{board}")]
pub async fn get_json(
    path: Path<(String, String)>,
    p: Data<Pool>,
) -> Result<impl Responder, ApiError> {
    let (_conn, board) =
        models::Board::from_route_lang_short(&path.0, &path.1, p.get_conn().await?).await?;

    res::json!(OK, board)
}
