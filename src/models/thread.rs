use super::*;

#[derive(FromRow, Debug)]
pub struct Thread {
    pub id: u64,
    pub owner: Option<u64>,
    pub ip: u64,
    pub board_id: u64,
    pub post_id: u64,
    pub post_num: u64,
    pub sticky: u32,
    pub secret: u64,
    pub count: u64,
    pub closed: u8,
    pub endless: bool,
    #[field_default]
    pub posts: Option<Vec<Post>>,
    pub lasthit: NaiveDateTime,
}

pub struct IThread {
    pub owner: Option<u64>,
    pub ip: u64,
    pub board_id: u64,
    pub post_id: u64,
    pub post_num: u64,
    pub sticky: u32,
    pub secret: u64,
    pub endless: bool,
}

const SELECT_ALIVE_THREAD_ON_BOARD_BY_NUM: &str = r"
    SELECT
        `threads`.*
    FROM
        (SELECT * FROM `threads` WHERE `threads`.`board_id` = :board_id ORDER BY `threads`.`sticky` DESC, `threads`.`lasthit` DESC LIMIT :count) as `threads`
    WHERE
        `threads`.`post_num` = :post_num";

const SELECT_ALIVE_THREAD_POST_ON_BOARD_BY_NUM: &str = r"
    SELECT
        `threads`.`id` as `thread_id`,
        `threads`.`owner` as `owner`,
        `threads`.`ip` as `ip`,
        `threads`.`board_id` as `board_id`,
        `threads`.`post_id` as `post_id`,
        `threads`.`post_num` as `post_num`,
        `threads`.`closed` as `thread_closed`,
        `threads`.`sticky` as `thread_sticky`,
        `threads`.`secret` as `thread_secret`,
        `threads`.`endless` as `thread_endless`,
        `threads`.`count` as `count`,
        `threads`.`lasthit` as `thread_lasthit`,
        `posts`.`pinned` as `post_pinned`,
        `posts`.`sage` as `post_sage`,
        `posts`.`op` as `post_op`,
        `posts`.`moder` as `post_moder`,
        `posts`.`trip` as `post_trip`,
        `posts`.`subject` as `post_subject`,
        `posts`.`name` as `post_name`,
        `posts`.`email` as `post_email`,
        `posts`.`message` as `post_message`,
        `posts`.`good` as `post_good`,
        `posts`.`bad` as `post_bad`,
        `posts`.`at` as `at`
    FROM
        (SELECT * FROM `threads` WHERE `threads`.`board_id` = :board_id ORDER BY `threads`.`sticky` DESC, `threads`.`lasthit` DESC LIMIT :count) as `threads`
    INNER JOIN `posts`
        ON `posts`.`id`=`threads`.`post_id`
    WHERE
        `threads`.`post_num` = :post_num
";

const SELECT_THREADS_POST_LIMIT_NUM_QUERY: &str = r"
    SELECT
        `threads`.`id` as `thread_id`,
        `threads`.`owner` as `owner`,
        `threads`.`ip` as `ip`,
        `threads`.`board_id` as `board_id`,
        `threads`.`post_id` as `post_id`,
        `threads`.`post_num` as `post_num`,
        `threads`.`closed` as `thread_closed`,
        `threads`.`sticky` as `thread_sticky`,
        `threads`.`secret` as `thread_secret`,
        `threads`.`endless` as `thread_endless`,
        `threads`.`count` as `count`,
        `threads`.`lasthit` as `thread_lasthit`,
        `posts`.`pinned` as `post_pinned`,
        `posts`.`sage` as `post_sage`,
        `posts`.`op` as `post_op`,
        `posts`.`moder` as `post_moder`,
        `posts`.`trip` as `post_trip`,
        `posts`.`subject` as `post_subject`,
        `posts`.`name` as `post_name`,
        `posts`.`email` as `post_email`,
        `posts`.`message` as `post_message`,
        `posts`.`good` as `post_good`,
        `posts`.`bad` as `post_bad`,
        `posts`.`at` as `at`
    FROM
        `threads`
    INNER JOIN `posts`
        ON `posts`.`id`=`threads`.`post_id`
    WHERE
        `threads`.`board_id` = :board_id
    ORDER BY
        `threads`.`sticky` DESC,
        `threads`.`lasthit` DESC
    LIMIT :count
";

const SELECT_THREADS_POST_OFFSET_LIMIT: &str = r"
    SELECT
        `threads`.`id` as `thread_id`,
        `threads`.`owner` as `owner`,
        `threads`.`ip` as `ip`,
        `threads`.`board_id` as `board_id`,
        `threads`.`post_id` as `post_id`,
        `threads`.`post_num` as `post_num`,
        `threads`.`closed` as `thread_closed`,
        `threads`.`sticky` as `thread_sticky`,
        `threads`.`endless` as `thread_endless`,
        `threads`.`secret` as `thread_secret`,
        `threads`.`count` as `count`,
        `threads`.`lasthit` as `thread_lasthit`,
        `posts`.`pinned` as `post_pinned`,
        `posts`.`sage` as `post_sage`,
        `posts`.`op` as `post_op`,
        `posts`.`moder` as `post_moder`,
        `posts`.`trip` as `post_trip`,
        `posts`.`subject` as `post_subject`,
        `posts`.`name` as `post_name`,
        `posts`.`email` as `post_email`,
        `posts`.`message` as `post_message`,
        `posts`.`good` as `post_good`,
        `posts`.`bad` as `post_bad`,
        `posts`.`at` as `at`
    FROM
        `threads`
    INNER JOIN `posts`
        ON `posts`.`id`=`threads`.`post_id`
    WHERE
        `threads`.`board_id` = :board_id
    ORDER BY
        `threads`.`sticky` DESC,
        `threads`.`lasthit` DESC
    LIMIT :count
    OFFSET :offset
";

impl IThread {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Thread), MysqlError> {
        let conn = conn
            .drop_exec(
                "INSERT INTO `threads` (owner, ip, board_id, post_id, post_num, sticky, endless, secret)
            VALUES (:owner, :ip, :board_id, :post_id, :post_num, :sticky, :endless, :secret)",
                params! {
                    "owner"      => &self.owner,
                    "ip"         => &self.ip,
                    "board_id"   => &self.board_id,
                    "post_id"    => &self.post_id,
                    "post_num"   => &self.post_num,
                    "endless"    => &self.endless,
                    "sticky"     => &self.sticky,
                    "secret"     => &self.secret,
                },
            )
            .await?;

        conn.first_exec(
            "SELECT * FROM `threads` WHERE id=(SELECT LAST_INSERT_ID())",
            (),
        )
        .await
        .map(|(c, v)| (c, v.map(Thread::from_row).unwrap()))
    }
}

impl Thread {
    /* dynamic */
    pub async fn close(&mut self, conn: Conn) -> Result<Conn, MysqlError> {
        self.closed = 1;
        conn.drop_exec(
            "UPDATE `threads` SET `closed`=1 WHERE `id`=:id",
            params! { "id" => self.id },
        )
        .await
    }

    pub async fn open(&mut self, conn: Conn) -> Result<Conn, MysqlError> {
        self.closed = 0;
        conn.drop_exec(
            "UPDATE `threads` SET `closed`=0 WHERE `id`=:id",
            params! { "id" => self.id },
        )
        .await
    }

    pub async fn pin(&mut self, conn: Conn) -> Result<Conn, MysqlError> {
        self.sticky = 1;
        conn.drop_exec(
            "UPDATE `threads` SET `sticky`=1 WHERE `id`=:id",
            params! { "id" => self.id },
        )
        .await
    }

    pub async fn make_endless(&mut self, conn: Conn) -> Result<Conn, MysqlError> {
        self.endless = true;
        conn.drop_exec(
            "UPDATE `threads` SET `endless`=1 WHERE `id`=:id",
            params! { "id" => self.id },
        )
        .await
    }

    pub async fn unpin(&mut self, conn: Conn) -> Result<Conn, MysqlError> {
        self.sticky = 0;
        conn.drop_exec(
            "UPDATE `threads` SET `sticky`=0 WHERE `id`=:id",
            params! { "id" => self.id },
        )
        .await
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
            "DELETE FROM `posts` WHERE `parent_id`=:parent_id AND `num` IN (",
            &digits_str,
            ")"
        ].concat();

        // execute
        conn.drop_exec(query_str, params! { "parent_id" => self.id })
            .await
    }

    pub async fn get_count_from_num(
        &self,
        num: u64,
        conn: Conn,
    ) -> Result<(Conn, u64), MysqlError> {
        let (conn, row): (_, Option<Row>) = conn.first_exec("SELECT COUNT(`id`) as `count` FROM `posts` WHERE `parent_id`=:parent_id AND `num` >= :num", params!{
            "parent_id" => self.id,
            "num"       => num
        }).await?;

        Ok((conn, row.unwrap().get(0).unwrap()))
    }

    pub async fn increment_count(&self, bump: bool, conn: Conn) -> Result<Conn, MysqlError> {
        let set = if bump {
            "`count`=`count`+1, `lasthit`=CURRENT_TIMESTAMP"
        } else {
            "`count`=`count`+1"
        };

        conn.drop_exec(
            &format!("UPDATE `threads` SET {set} WHERE `id`=:id", set = set),
            params! { "id" => self.id },
        )
        .await
    }

    pub async fn decrement_count(&self, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec("UPDATE `threads` SET `count` = `count` - 1 WHERE `id` = :id", params! {
            "id" => self.id
        })
        .await
    }

    pub async fn get_all_counts_from_direction_iter<I>(
        mut directions: I,
        conn: Conn
    ) -> Result<(Conn, Vec<(u64, u64, u64)>), MysqlError>
        where I: Iterator<Item = (u64, u64)> {
        let mut boards = String::new();
        let mut nums = String::new();

        macro_rules! push {
            ($target:expr, $value:expr) => {
                $target.push_str(&$value);
                $target.push(',');
            };
        }

        match directions.next() {
            None => return Ok((conn, Vec::new())),
            Some((board, num)) => {
                push!(boards, board.to_string());
                push!(nums, num.to_string());
            }
        }

        for (board, num) in directions {
            push!(boards, board.to_string());
            push!(nums, num.to_string());
        }
        boards.pop();
        nums.pop();

        conn.prep_exec(["SELECT `threads`.`board_id`,`threads`.`post_num`, `threads`.`count` FROM `threads` WHERE `threads`.`board_id` IN (", &boards,") AND `threads`.`post_num` IN (", &nums,")"].concat(),
            ()
        )
        .await?
        .map_and_drop(|row| (row.get(0).unwrap(), row.get(1).unwrap(), row.get(2).unwrap()))
        .await
    }

    pub async fn get_replies_with_files_by_page(
        &self,
        page: u32,
        limit: u32,
        conn: Conn,
    ) -> Result<(Conn, Vec<Post>), MysqlError> {
        let (conn, mut posts) = self.get_replies_page(page, limit, conn).await?;

        if posts.is_empty() {
            return Ok((conn, Vec::new()));
        }

        let conn = models::post::load_files(&mut posts, conn).await?;

        Ok((conn, posts))
    }

    pub async fn get_replies_with_files_by_page_from_num(
        &self,
        page: u32,
        num: u64,
        limit: u32,
        conn: Conn,
    ) -> Result<(Conn, Vec<Post>), MysqlError> {
        let (conn, mut posts) = self.get_replies_page_from_num(page, num, limit, conn).await?;

        if posts.is_empty() {
            return Ok((conn, Vec::new()));
        }

        let conn = models::post::load_files(&mut posts, conn).await?;

        Ok((conn, posts))
    }

    pub async fn get_replies_with_files_from_num(
        &self,
        num: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<Post>), MysqlError> {
        let (conn, mut posts) = Self::get_replies_from_num(self, num, conn).await?;

        if posts.is_empty() {
            return Ok((conn, Vec::new()));
        }

        let conn = models::post::load_files(&mut posts, conn).await?;

        Ok((conn, posts))
    }

    pub async fn get_replies_from_num(
        &self,
        num: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<Post>), MysqlError> {
        conn.prep_exec(
            "SELECT * FROM `posts` WHERE `parent_id`=:parent_id AND `num` >= :num",
            params! {
                "parent_id" => self.id,
                "num"       => num
            },
        )
        .await?
        .map_and_drop(|row| Post::from_row(row))
        .await
    }

    pub async fn get_replies(&self, conn: Conn) -> Result<(Conn, Vec<Post>), MysqlError> {
        conn.prep_exec(
            "SELECT * FROM `posts` WHERE `parent_id`=:parent_id",
            params! {
                "parent_id" => self.id
            },
        )
        .await?
        .map_and_drop(Post::from_row)
        .await
    }

    pub async fn get_replies_page_from_num(&self, page: u32, num: u64, limit: u32, conn: Conn) -> Result<(Conn, Vec<Post>), MysqlError> {
        conn.prep_exec(
            "SELECT `posts`.* FROM (
                SELECT id, num FROM `posts` WHERE `parent_id`=:parent_id LIMIT :limit OFFSET :offset
            ) AS `i`
                JOIN `posts` ON `i`.`id`=`posts`.`id`
                WHERE `i`.`num` >= :num",
            params! {
                "parent_id" => self.id,
                "offset" => page * limit,
                "limit" => limit, 
                "num" => num,
            },
        )
        .await?
        .map_and_drop(Post::from_row)
        .await
    }

    pub async fn get_replies_page(&self, page: u32, limit: u32, conn: Conn) -> Result<(Conn, Vec<Post>), MysqlError> {
        conn.prep_exec(
            "SELECT * FROM `posts` WHERE `parent_id`=:parent_id LIMIT :limit OFFSET :offset",
            params! {
                "parent_id" => self.id,
                "offset" => page * limit,
                "limit" => limit
            },
        )
        .await?
        .map_and_drop(Post::from_row)
        .await
    }

    /* static */
    pub async fn delete_by_post_id(id: u64, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "DELETE FROM `threads` WHERE `post_id`=:post_id",
            params! { "post_id" => id },
        )
        .await
    }

    pub async fn find_by_board_id_and_post_num_from_last(
        board_id: u64,
        post_num: u64,
        count: u64,
        conn: Conn,
    ) -> Result<(Conn, Option<Thread>), MysqlError> {
        conn.first_exec(
            SELECT_ALIVE_THREAD_ON_BOARD_BY_NUM,
            params! {
                "board_id" => board_id,
                "post_num" => post_num,
                "count"    => count
            },
        )
        .await
        .map(|(c, v)| (c, v.map(Self::from_row)))
    }

    pub async fn find_by_board_id_and_post_num_from_last_with_post(
        board_id: u64,
        post_num: u64,
        count: u64,
        conn: Conn,
    ) -> Result<(Conn, Option<(Thread, Post)>), MysqlError> {
        conn.first_exec(
            SELECT_ALIVE_THREAD_POST_ON_BOARD_BY_NUM,
            params! {
                "board_id" => board_id,
                "post_num" => post_num,
                "count"    => count
            },
        )
        .await
        .map(|(c, v)| (c, v.map(super::row_to_thread_post)))
    }

    pub async fn find_by_post_id(post_id: u64, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec(
            "SELECT * FROM `threads` WHERE `post_id`=:post_id",
            params! { "post_id" => post_id }
        )
        .await
        .map(|(c, v)| (c, v.map(FromRow::from_row)))
    }

    pub async fn find(id: u64, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec(
            "SELECT * FROM `threads` WHERE `id`=:id",
            params! { "id" => id }
        )
        .await
        .map(|(c, v)| (c, v.map(FromRow::from_row)))
    }

    pub async fn last_threads_on_board_with_post_as_reply_to_value(
        board_id: u64,
        count: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<Value>), MysqlError> {
        conn.prep_exec(
            SELECT_THREADS_POST_LIMIT_NUM_QUERY,
            params! {
                "board_id" => board_id,
                "count"    => count
            },
        )
        .await?
        .map_and_drop(super::row_to_thread_post_as_reply_to_value)
        .await
    }

    pub async fn last_threads_on_board_with_post_as_reply(
        board_id: u64,
        count: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<Thread>), MysqlError> {
        conn.prep_exec(
            SELECT_THREADS_POST_LIMIT_NUM_QUERY,
            params! {
                "board_id" => board_id,
                "count"    => count
            },
        )
        .await?
        .map_and_drop(super::row_to_thread_post_as_reply)
        .await
    }

    pub async fn all_on_board_page_with_oppost(
        board_id: u64,
        offset: u64,
        count: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<(Self, Post)>), MysqlError> {
        conn.prep_exec(
            SELECT_THREADS_POST_OFFSET_LIMIT,
            params! {
                "board_id" => board_id,
                "count"    => count,
                "offset"   => offset
            },
        )
        .await?
        .map_and_drop(super::row_to_thread_post)
        .await
    }

    pub async fn last_threads_on_board_with_post(
        board_id: u64,
        count: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<(Self, Post)>), MysqlError> {
        conn.prep_exec(
            SELECT_THREADS_POST_LIMIT_NUM_QUERY,
            params! {
                "board_id" => board_id,
                "count"    => count
            },
        )
        .await?
        .map_and_drop(super::row_to_thread_post)
        .await
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
            ["SELECT `id` FROM `posts` WHERE `parent_id`=:parent_id AND `num` IN (", &ids_str, ")"].concat(),
            params! {
                "parent_id" => self.id
            }
        )
        .await?
        .map_and_drop(|row| row.get(0).unwrap())
        .await
    }

    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `threads`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
}

impl Serialize for Thread {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Thread", 7)?;
        state.serialize_field("num", &self.post_num)?;
        state.serialize_field("count", &self.count)?;
        state.serialize_field("pinned", &self.sticky)?;
        state.serialize_field("posts", &self.posts)?;
        state.serialize_field("endless", &self.endless)?;
        state.serialize_field("closed", &self.closed)?;
        state.serialize_field("lasthit", &self.lasthit.timestamp())?;


        state.end()
    }
}
