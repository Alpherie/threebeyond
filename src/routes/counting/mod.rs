use super::*;
use std::collections::HashSet;

#[get("[{directions:.*}]")]
pub async fn get(
    p: Path<String>,
    pool: Data<Pool>,
    app_state: Data<AppState>,
) -> Result<impl Responder, ApiError> {
    use serde_json::{Map as JMap, Value as JValue};

    let conn = pool.get_conn().await?;
    let mut directions = Vec::new();
    let mut counts = Vec::new();
    let mut boards = HashSet::new();
    let mut langs = HashSet::new();

    macro_rules! err {
        () => {
            ApiError::new(
                StatusCode::BAD_REQUEST,
                Some("URL_PARSING"),
                "DIRECTIONS_BAD_FORMAT",
                "Format for directions is invalid.",
                None,
            )
        };
    }

    let lock = app_state.threads_count_cache.lock().await;
    let split: Vec<&str> = p.split(",").collect();
    if split.len() > config::THREAD_COUNT_MAX_PER_REQUEST {
        Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            Some("URL_PARSING"),
            "DIRECTIONS_LIMIT",
            "You passeed more directions that limits is allowed.",
            Some(json!({ "limit": config::THREAD_COUNT_MAX_PER_REQUEST })),
        ))?
    }

    for entry in split {
        let parts: Vec<&str> = entry.split("/").collect();
        if parts.len() != 3 || parts[0].len() > 8 || parts[1].len() > 16 {
            Err(err!())?
        }

        let num: u64 = parts[2].parse().map_err(|_| err!())?;

        let key = (parts[0], parts[1], num);
        // find
        // FIXME: clone
        if let Some((count, _)) = lock.get(&(key.0.into(), key.1.into(), key.2)) {
            counts.push(((key.0.to_string(), key.1.to_string(), key.2), *count));
        } else {
            directions.push((key.0, key.1, key.2));
        }

        if !boards.contains(key.1) {
            boards.insert(key.1);
        }

        if !langs.contains(key.0) {
            langs.insert(key.0);
        }
    }
    drop(lock);

    let mut final_iter: Box<dyn Iterator<Item = ((String, String, u64), u64)>> =
        Box::new(counts.into_iter());

    if !directions.is_empty() {
        let (conn, boards_short_ids) =
            models::Board::get_all_ids_langs_shorts_from_lang_short_iter(
                langs.into_iter(),
                boards.into_iter(),
                conn,
            )
            .await?;

        // link board_ids to short back
        let mut board_id_to_lang_short = HashMap::new();
        let mut board_short_to_id_lang = HashMap::new();
        for (id, lang, short) in boards_short_ids {
            board_short_to_id_lang.insert(short.clone(), (id, lang.clone()));
            board_id_to_lang_short.insert(id, (lang, short));
        }

        let directions =
            directions
                .into_iter()
                .filter_map(|(_, board_short, num)| {
                    match board_short_to_id_lang.get(board_short) {
                        Some(v) => Some((v.0, num)),
                        None => None,
                    }
                });

        let (conn, remain) =
            models::Thread::get_all_counts_from_direction_iter(directions, conn).await?;

        if !remain.is_empty() {
            // construct valid data
            let output: Vec<((String, String, u64), u64)> = remain
                .into_iter()
                .map(|(board_id, num, count)| {
                    let back = board_id_to_lang_short[&board_id].clone();

                    ((back.0, back.1, num), count)
                })
                .collect();

            // cache
            app_state.set_thread_counts_cache(&output).await;

            final_iter = Box::new(final_iter.chain(output.into_iter()));
        }
    }

    // construct final Json Map
    let map: JMap<_, _> = final_iter
        .map(|((lang, board, num), count)| {
            (
                [&lang, "/", &board, "/", &num.to_string()].concat(),
                Value::Number(count.into()),
            )
        })
        .collect();

    res::json!(map)
}
