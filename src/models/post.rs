use super::*;
use std::collections::BTreeMap;

#[derive(FromRow, Debug)]
pub struct Post {
    pub id: u64,
    pub owner: Option<u64>, // token_id
    pub ip: u64,
    pub board_id: u64,
    pub parent_id: Option<u64>,
    pub parent_num: Option<u64>,
    pub pinned: bool,
    pub num: u64,
    pub sage: bool,
    pub op: bool,
    pub moder: bool,
    pub trip: Option<String>,
    pub subject: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub message: String,
    pub good: u64,
    pub bad: u64,
    pub at: NaiveDateTime,

    #[field_default]
    pub files: Option<Vec<File>>,
}

pub struct IPost {
    pub owner: Option<u64>, // token_id
    pub ip: u64,
    pub board_id: u64,
    pub parent_id: Option<u64>,
    pub parent_num: Option<u64>,
    pub pinned: bool,
    pub sage: bool,
    pub op: bool,
    pub moder: bool,
    pub trip: Option<String>,
    pub subject: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub message: String,
}

impl IPost {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Post), MysqlError> {
        let conn = conn.drop_exec("
            UPDATE `boards`
                SET count=@num := count+1
                WHERE `id`=:board_id", params!{ "board_id" => self.board_id }).await?
        .drop_exec("INSERT INTO `posts` (owner, ip, board_id, parent_id, parent_num, num, pinned, sage, op, subject, trip, name, email, message)
            VALUES (
                :owner,
                :ip,
                :board_id,
                :parent_id,
                :parent_num,
                @num,
                :pinned,
                :sage,
                :op,
                :subject,
                :trip,
                :name,
                :email,
                :message)",
            params! {
                "owner"      => &self.owner,
                "ip"         => &self.ip,
                "board_id"   => &self.board_id,
                "parent_id"  => &self.parent_id,
                "parent_num" => &self.parent_num,
                "pinned"     => &self.pinned,
                "sage"       => &self.sage,
                "op"         => &self.op,
                "subject"    => &self.subject,
                "trip"       => &self.trip,
                "name"       => &self.name,
                "email"      => &self.email,
                "message"    => &self.message
            }).await?;

        conn.first_exec(
            "SELECT * FROM `posts` WHERE ID=(SELECT LAST_INSERT_ID())",
            (),
        )
        .await
        .map(|(c, v)| (c, Post::from_row(v.unwrap())))
    }
}

impl Post {
    pub async fn find_all_ids_for_thread(thread_id: u64, conn: Conn) -> Result<(Conn, Vec<u64>), MysqlError> {
        conn.prep_exec(
            "SELECT `id` FROM `posts` WHERE `parent_id`=:thread_id",
            params! {
                "thread_id" => thread_id
            }
        ).await?
        .map_and_drop(|row| row.get(0).unwrap())
        .await
    }

    pub async fn find_all_where_num_in_on_board(nums: &[u64], board_id: u64, conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        let first = match nums.get(0) {
            Some(v) => v,

            None => return Ok((conn, Vec::new())),
        };

        // calculate first number digits count using logarithm hack
        let digits = (*first as f64).log10() as u64;

        // prepare query string
        let mut digits_str = String::with_capacity((digits as usize + 1) * nums.len());

        // bind values
        digits_str.push_str(&first.to_string());
        digits_str.push(',');
        for id in nums {
            digits_str.push_str(&id.to_string());
            digits_str.push(',');
        }

        digits_str.pop();

        let query_str = [
            "SELECT * FROM `posts` WHERE `board_id`=:board_id AND `num` IN (",
            &digits_str,
            ") ORDER BY `id` DESC",
        ]
        .concat();

        // execute
        conn.prep_exec(query_str, params! { "board_id" => board_id })
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
    pub async fn files<'a>(
        &'a mut self,
        mut conn: Conn,
    ) -> Result<(Conn, &'a Vec<File>), MysqlError> {
        if let None = self.files {
            let (c, files) = self.get_files(conn).await?;

            self.files = Some(files);
            conn = c;
        };

        Ok((conn, self.files.as_ref().unwrap()))
    }

    async fn get_files(&self, conn: Conn) -> Result<(Conn, Vec<File>), MysqlError> {
        models::File::related_to_post_id(self.id, conn).await
    }

    pub async fn pin(&mut self, conn: Conn) -> Result<Conn, MysqlError> {
        self.pinned = true;
        conn.drop_exec(
            "UPDATE `posts` SET `pinned`=1 WHERE `id`=:id",
            params! { "id" => self.id },
        )
        .await
    }

    pub async fn unpin(&mut self, conn: Conn) -> Result<Conn, MysqlError> {
        self.pinned = false;
        conn.drop_exec(
            "UPDATE `posts` SET `pinned`=0 WHERE `id`=:id",
            params! { "id" => self.id },
        )
        .await
    }

    /* dynamic */
    pub async fn delete(&self, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "DELETE FROM `posts` WHERE `id`=:id",
            params! {"id" => self.id},
        )
        .await
    }

    pub async fn delete_by_parent_id(id: u64, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "DELETE FROM `posts` WHERE `parent_id`=:id",
            params! {"id" => id},
        )
        .await
    }

    // pub async fn delete_and_unlink(&self, conn: Conn) -> Result<Conn, MysqlError> {
    //     // delete
    //     let conn = self.delete(conn).await?;

    //     // decrement counters
    //     conn
    //         .drop_exec("UPDATE `boards` SET `count` = `count` - 1 WHERE `id`=:id", params!{ "id" => self.board_id }).await?
    //         .drop_exec("UPDATE `thread` SET `count` = `count` - 1 WHERE `id`=:id", params!{ "id" => self.parent_id }).await?
    //         .drop_exec("DELETE FROM FILES WHERE `post_id`=:id", params!{ "id" => self.id }).await
    // }

    /* static */
    pub async fn find_by_board_id_and_post_num_in_thread_by_num(board_id: u64, post_num: u64, parent_num: u64, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec(
            "SELECT * FROM `posts` WHERE `board_id`=:board_id AND `num`=:post_num AND `parent_num`=:parent_num",
            params! {
                "board_id" => board_id,
                "post_num" => post_num,
                "parent_num" => parent_num
            }
        )
        .await
    }

    pub async fn find_on_board_id_by_num(
        board_id: u64,
        num: u64,
        conn: Conn,
    ) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec(
            "SELECT * FROM `posts` WHERE `board_id`=:board_id AND `num`=:num",
            params! {
                "board_id" => board_id,
                "num"      => num
            },
        )
        .await
    }

    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `posts`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }

    pub async fn last_replies_for_threads(
        threads: &[u64],
        count: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<Self>), MysqlError> {
        /* here is dirty way to build `where in` query... */
        if threads.len() == 0 {
            return Ok((conn, Vec::new()));
        }

        // calculate first number digits count using logarithm hack
        let digits = (threads[0] as f64).log10() as u64;

        // prepare query string
        let mut digits_str = String::with_capacity((digits as usize + 1) * threads.len());

        // bind values
        for id in threads {
            digits_str.push_str(&id.to_string());
            digits_str.push(',');
        }

        digits_str.pop();

        let query_str = [
            "SELECT * FROM (SELECT `id` FROM (SELECT `posts`.`id`, ROW_NUMBER() OVER(PARTITION BY `posts`.`parent_id` ORDER BY `posts`.`id` DESC) AS `rn` FROM `posts` WHERE `parent_id` IN (",
            &digits_str,
            ")) AS `A` WHERE `A`.`rn` <= :count) AS `S` INNER JOIN `posts` ON `posts`.`id`=`S`.`id`"
        ].concat();

        // execute
        conn.prep_exec(query_str, params! { "count" => count })
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
}

pub async fn load_files(posts: &mut Vec<Post>, conn: Conn) -> Result<Conn, MysqlError> {
    let (conn, files) = File::all_for_posts(posts.iter().map(|v| v.id), conn).await?;

    let mut map = posts
        .iter_mut()
        .map(|v| (v.id, v))
        .collect::<BTreeMap<_, _>>();

    let mut post_index = *map.keys().next().expect("no first key");
    let mut post_ref = map.get_mut(&post_index).unwrap();

    for file in files {
        if post_index != file.post {
            post_ref = map.get_mut(&file.post).expect("no post parent for file");

            post_index = file.post;
        }

        match post_ref.files.as_mut() {
            Some(v) => v.push(file),

            None => {
                post_ref.files = Some({
                    let mut v = Vec::with_capacity(4);

                    v.push(file);

                    v
                })
            }
        }
    }

    Ok(conn)
}

impl Serialize for Post {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Post", 15)?;
        state.serialize_field("num", &self.num)?;
        state.serialize_field("pinned", &self.pinned)?;
        state.serialize_field("sage", &self.sage)?;
        state.serialize_field("op", &self.op)?;
        state.serialize_field("mod", &self.moder)?;
        state.serialize_field("subject", &self.subject)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("comment", &self.message)?;
        state.serialize_field(
            "thread",
            &match self.parent_num {
                Some(v) => v,
                None => self.num,
            },
        )?;
        state.serialize_field("trip", &self.trip)?;
        state.serialize_field("good", &self.good)?;
        state.serialize_field("bad", &self.bad)?;
        state.serialize_field("at", &self.at.timestamp())?;
        state.serialize_field(
            "files",
            match &self.files {
                Some(v) => v.as_slice(),
                None => &[],
            },
        )?;
        state.end()
    }
}
