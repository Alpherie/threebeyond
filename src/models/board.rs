use super::*;
use mysql_async::{Value};

#[derive(FromRow, Serialize, Debug, Clone)]
pub struct Board {
    #[serde(skip)]
    pub id: u64,
    pub lang: String,
    pub short: String,
    pub name: String,
    pub description: String,
    pub rules: String,
    pub pages_count: u64,
    pub per_page: u64,
    pub last_replies: u64,
    pub count: u64,
    pub bumplimit: u32,
    pub thread_creating_limited: u8,
    pub slowmode: Option<u64>,
    pub last_thread_created: NaiveDateTime,
    pub op_oppost_enabled: bool,
    pub op_deletion_enabled: bool,
    pub tripcode_enabled: bool
}

pub struct IBoard<'a> {
    pub lang: &'a str,
    pub short: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub rules: &'a str,
    pub pages_count: u64,
    pub per_page: u64,
    pub last_replies: u64,
    pub bumplimit: u32,
    pub thread_creating_limited: u8,
    pub slowmode: Option<u64>,
    pub last_thread_created: NaiveDateTime,
    pub op_oppost_enabled: bool,
    pub op_deletion_enabled: bool,
    pub tripcode_enabled: bool
}

impl IBoard<'_> {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Board), MysqlError> {
        let conn = conn.drop_exec("INSERT INTO `boards` (short, lang, name, description, rules, pages_count, per_page, last_replies, bumplimit, thread_creating_limited, slowmode, last_thread_created, op_oppost_enabled, op_deletion_enabled, tripcode_enabled) VALUES (:short, :lang, :name, :description, :rules, :pages_count, :per_page, :last_replies, :bump_limit, :thread_creating_limited, :slowmode, :last_thread_created, :op_oppost_enabled, :op_deletion_enabled, :tripcode_enabled)",
            params! {
                "short"        => &self.short,
                "lang"         => &self.lang,
                "name"         => &self.name,
                "description"  => &self.description,
                "rules"        => &self.rules,
                "pages_count"  => &self.pages_count,
                "per_page"     => &self.per_page,
                "last_replies" => &self.last_replies,
                "bump_limit"   => &self.bumplimit,
                "thread_creating_limited" => &self.thread_creating_limited,
                "slowmode"      => &self.slowmode,
                "last_thread_created" => &self.last_thread_created,
                "op_oppost_enabled" => &self.op_oppost_enabled,
                "op_deletion_enabled" => &self.op_deletion_enabled,
                "tripcode_enabled" => &self.tripcode_enabled,
            }).await?;

        conn.first_exec(
            "SELECT * FROM `boards` WHERE ID=(SELECT LAST_INSERT_ID())",
            (),
        )
        .await
        .map(|(c, v)| (c, v.unwrap()))
    }
}

impl Board {
    pub async fn delete(&self, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "DELETE FROM `boards` WHERE `id`=:id",
            params! {
                "id" => &self.id,
            }
        )
        .await
    }

    pub async fn save_all(&self, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "UPDATE `boards` SET `id` = :id, `lang` = :lang, `short` = :short, `name` = :name, `description` = :description, `rules` = :rules, `pages_count` = :pages_count, `per_page` = :per_page, `last_replies` = :last_replies, `count` = :count, `bumplimit` = :bumplimit, `thread_creating_limited` = :thread_creating_limited, `slowmode` = :slowmode, `last_thread_created` = :last_thread_created, `op_oppost_enabled` = :op_oppost_enabled, `op_deletion_enabled` = :op_deletion_enabled, tripcode_enabled = :tripcode_enabled WHERE `id`=:id",
            params! {
                "id" => &self.id,
                "lang" => &self.lang,
                "short" => &self.short,
                "name" => &self.name,
                "description" => &self.description,
                "rules" => &self.rules,
                "pages_count" => &self.pages_count,
                "per_page" => &self.per_page,
                "last_replies" => &self.last_replies,
                "count" => &self.count,
                "bumplimit" => &self.bumplimit,
                "thread_creating_limited" => &self.thread_creating_limited,
                "slowmode" => &self.slowmode,
                "last_thread_created" => &self.last_thread_created,
                "op_oppost_enabled" => &self.op_oppost_enabled,
                "op_deletion_enabled" => &self.op_deletion_enabled,
                "tripcode_enabled" => &self.tripcode_enabled,
            }
        )
        .await
    }

    pub async fn update_last_thread(&self, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec("UPDATE `boards` SET `last_thread_created` = CURRENT_TIMESTAMP WHERE `id` = :id", params! {
            "id" => self.id
        })
        .await
    }

    pub async fn get_all_ids_langs_shorts_from_lang_short_iter<'a, L, B>(langs_iter: L, boards_iter: B, conn: Conn) -> Result<(Conn, Vec<(u64, String, String)>), MysqlError> where B: Iterator<Item = &'a str> + 'a, L: Iterator<Item = &'a str> + 'a {
        let mut preparation_short = String::new();
        let mut preparation_lang = String::new();
        let mut vec: Vec<Value> = boards_iter.map(|s| {
            preparation_short.push_str("?,");
            Value::Bytes(s.as_bytes().to_vec())
        }).collect();
        preparation_short.pop();
        
        vec.extend(langs_iter.map(|s| {
            preparation_lang.push_str("?,");
            Value::Bytes(s.as_bytes().to_vec())
        }));
        preparation_lang.pop();
        let params = mysql_async::Params::Positional(vec);

        conn.prep_exec(["SELECT `id`, `lang`, `short` FROM `boards` WHERE `short` IN (", &preparation_short,") AND `lang` IN (", &preparation_lang, ")"].concat(), params)
        .await?
        .map_and_drop(|row| (row.get(0).unwrap(), row.get(1).unwrap(), row.get(2).unwrap()))
        .await
    }

    pub async fn get_reports_page(&self, page: u64, conn: Conn) -> Result<(Conn, Vec<models::Report>), MysqlError> {
        models::Report::get_page_for_board_id(page, self.id, conn).await
    }

    pub async fn get_reports_page_unsolved(&self, page: u64, conn: Conn) -> Result<(Conn, Vec<models::Report>), MysqlError> {
        models::Report::get_page_unsolved_for_board_id(page, self.id, conn).await
    }

    pub async fn delete_multiple_posts_by_nums(&self, list: &[u64], conn: Conn) -> Result<Conn, MysqlError> {
        // prepare request
        /* here is a dirty way to build `where in` query... */
        // calculate first number digits count using logarithm hack
        let _first = list[0];
        let digits = (list[0] as f64).log10() as u64;

        // prepare query string
        let mut digits_str = String::with_capacity((digits as usize + 1) * list.len());

        // bind values
        for id in list {
            digits_str.push_str(&id.to_string());
            digits_str.push(',');
        }

        digits_str.pop();

        let query_str = [
            "DELETE FROM `posts` WHERE `board_id`=:board_id AND `num` IN (",
            &digits_str,
            ")"
        ].concat();

        let query_str_threads = [
            "DELETE FROM `threads` WHERE `board_id`=:board_id AND `post_num` IN (",
            &digits_str,
            ")"
        ].concat();

        // execute
        conn.drop_exec(query_str, params! { "board_id" => self.id })
            .await?
            .drop_exec(query_str_threads, params! { "board_id" => self.id })
            .await
    }

    pub async fn decrement_count(&self, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec("UPDATE `boards` SET `count` = `count` - 1 WHERE `id` = :id", params! {
            "id" => self.id
        })
        .await
    }

    pub async fn increment_count(&self, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec("UPDATE `boards` SET `count` = `count` + 1 WHERE `id` = :id", params! {
            "id" => self.id
        })
        .await
    }

    pub async fn from_route_post_any(
        &self,
        num: u64,
        conn: Conn,
    ) -> Result<(Conn, Post), ApiError> {
        match models::Post::find_on_board_id_by_num(self.id, num, conn).await? {
            (_conn, None) => {
                Err(ApiError::post_not_found_on_board(&self.short, num))
            }

            (conn, Some(v)) => Ok((conn, v)),
        }
    }

    pub async fn from_route_in_thread_num_get_post_by_num(
        &self,
        thread_num: u64,
        post_num: u64,
        conn: Conn,
    ) -> Result<(Conn, Post), ApiError> {
        match models::Post::find_by_board_id_and_post_num_in_thread_by_num(
            self.id,
            post_num,
            self.per_page * self.pages_count,
            conn,
        )
        .await?
        {
            (_conn, None) => {
                Err(ApiError::post_not_found(&self.lang, &self.short, thread_num, post_num))
            }

            (conn, Some(v)) => Ok((conn, v)),
        }
    }

    pub async fn from_route_thread_last(
        &self,
        num: u64,
        conn: Conn,
    ) -> Result<(Conn, Thread), ApiError> {
        match models::Thread::find_by_board_id_and_post_num_from_last(
            self.id,
            num,
            self.per_page * self.pages_count,
            conn,
        )
        .await?
        {
            (_conn, None) => {
                Err(ApiError::thread_not_found(&self.short, num))
            }

            (conn, Some(v)) => Ok((conn, v)),
        }
    }

    pub async fn from_route_thread_post_last(
        &self,
        num: u64,
        conn: Conn,
    ) -> Result<(Conn, (Thread, Post)), ApiError> {
        match models::Thread::find_by_board_id_and_post_num_from_last_with_post(
            self.id,
            num,
            self.per_page * self.pages_count,
            conn,
        )
        .await?
        {
            (_conn, None) => Err(ApiError::thread_not_found(&self.short, num)),

            (conn, Some(v)) => Ok((conn, v)),
        }
    }

    async fn get_threads_on_page(
        &self,
        page: u64,
        any: bool,
        conn: Conn,
    ) -> Result<(Conn, Vec<Thread>), MysqlError> {
        // check if it's not available
        if self.pages_count < page && !any {
            return Ok((conn, Vec::new()));
        }

        // offset
        let offset = self.per_page * page;

        // get related threads
        models::Thread::all_on_board_page_with_oppost(self.id, offset, self.per_page, conn)
            .await
            .map(|(conn, t)| {
                (
                    conn,
                    t.into_iter()
                        .map(|(mut t, p)| {
                            t.posts = Some({
                                let mut v = Vec::with_capacity(t.count as usize);

                                v.push(p);

                                v
                            });

                            t
                        })
                        .collect(),
                )
            })
    }

    pub async fn get_all_posts_ids_by_post_nums(&self, post_nums: &[u64], conn: Conn) -> Result<(Conn, Vec<u64>), MysqlError> {
        if post_nums.is_empty() {
            return Ok((conn, Vec::default()))
        }
        let mut ids_str = String::new();
        for id in post_nums {
            ids_str.push_str(&id.to_string());
            ids_str.push(',');
        }
        ids_str.pop();
        conn.prep_exec(
            ["SELECT `id` FROM `posts` WHERE `board_id`=:board_id AND `num` IN (", &ids_str, ")"].concat(),
            params! {
                "board_id" => self.id
            }
        )
        .await?
        .map_and_drop(|row| row.get(0).unwrap())
        .await
    }

    pub async fn get_page(
        &self,
        page: u64,
        any: bool,
        conn: Conn,
    ) -> Result<(Conn, Vec<Thread>), MysqlError> {
        // get threads on page
        let (conn, mut threads) = self.get_threads_on_page(page, any, conn).await?;
        let tlen = threads.len();
        let thread_posts_ids = threads
            .iter()
            .map(|v| v.posts.as_ref().unwrap()[0].id)
            .collect::<Box<[_]>>();

        if tlen == 0 {
            return Ok((conn, Vec::new()));
        }

        // prepare sorted vector
        let mut tothreads = threads.iter_mut().map(|x| x).collect::<Box<_>>();
        tothreads.sort_by(|a, b| b.id.cmp(&a.id));

        // prepare keys for threads
        let thread_ids = tothreads.iter().map(|t| t.id).collect::<Box<_>>();

        // get replies
        let (conn, replies) =
            models::Post::last_replies_for_threads(&thread_ids, self.last_replies, conn).await?;

        // prepare keys for posts to get files
        let post_ids = thread_posts_ids
            .iter()
            .chain(replies.iter().map(|v| &v.id))
            .copied()
            .collect::<Box<[_]>>();

        let (conn, files) = models::File::all_for_posts(post_ids.iter().copied(), conn).await?;
        let mut files_map = std::collections::BTreeMap::new();
        let mut files_iter = files.into_iter();

        if let Some(n) = files_iter.next() {
            let mut v = Vec::with_capacity(8);
            let npost = n.post;
            v.push(n); // FIXME
            files_map.insert(npost, v);

            for file in files_iter {
                if let Some(v) = files_map.get_mut(&file.post) {
                    v.push(file);
                } else {
                    let file_post = file.post;

                    let mut v = Vec::with_capacity(8);
                    v.push(file);

                    files_map.insert(file_post, v);
                }
            }
        }

        let mut tothreads_map = tothreads.iter_mut().map(|t| (t.id, t)).collect::<HashMap<_, _>>();

        //
        for mut reply in replies.into_iter() {
            // get files
            reply.files = files_map.remove(&reply.id);

            // unwrap parent id
            let parent_id = reply.parent_id.unwrap();

            match tothreads_map.get_mut(&parent_id).map(|t| t.posts.as_mut()) {
                Some(Some(v)) => v.push(reply),
                None | Some(None) => panic!("Found a reply for an unknown thread"),
            }
        }

        // fill remaining threads with files
        for thread in tothreads.iter_mut() {
            let posts = thread.posts.as_mut().unwrap();

            let op = &mut posts[0];
            op.files = files_map.remove(&op.id);

            posts.sort_by(|left, right| left.id.cmp(&right.id));
        }

        drop(tothreads);

        Ok((conn, threads))
    }

    /* static */
    pub async fn from_route_lang_short(
        lang: &str,
        short: &str,
        conn: Conn,
    ) -> Result<(Conn, Self), ApiError> {
        match Self::find_by_short_for_lang(short, lang, conn).await? {
            (_conn, None) => Err(ApiError::board_not_found(short, lang)),

            (conn, Some(v)) => Ok((conn, v)),
        }
    }

    pub async fn find_by_short_for_lang(
        short: &str,
        lang: &str,
        conn: Conn,
    ) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec(
            "SELECT * FROM `boards` WHERE `short`=:short AND `lang`=:lang",
            params! { "short" => short, "lang" => lang },
        )
        .await
    }

    pub async fn all_for_lang(lang: &str, conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec(
            "SELECT * FROM `boards` WHERE `lang`=:lang",
            params! {"lang" => lang},
        )
        .await?
        .map_and_drop(Self::from_row)
        .await
    }

    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `boards`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
}
