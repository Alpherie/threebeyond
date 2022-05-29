use super::*;
use crate::utils::multipart::{AttachmentExtension, FileInfo};

#[derive(Serialize, Debug)]
pub struct File {
    pub uuid: Uuid,
    pub post: u64,
    pub name: String,
    pub extension: AttachmentExtension,
    pub width: u32,
    pub height: u32,
    pub size: usize,
}

pub struct IFile {
    pub uuid: Uuid,
    pub post: u64,
    pub name: String,
    pub extension: AttachmentExtension,
    pub width: u32,
    pub height: u32,
    pub size: usize,
}

impl FromRow for File {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(Self {
            uuid: row.get("uuid").unwrap(),
            post: row.get("post").unwrap(),
            name: row.get("name").unwrap(),
            extension: AttachmentExtension::from_str(&row.get::<String, _>("extension").unwrap())
                .expect("unknown extension for file"),
            width: row.get("width").unwrap(),
            height: row.get("height").unwrap(),
            size: row.get("size").unwrap(),
        })
    }
}

impl IFile {
    pub fn from_fi_for_post(post: u64, f: &utils::multipart::FileInfo) -> Self {
        Self {
            uuid: f.uuid,
            post: post,
            name: f.name.to_string(),
            extension: f.extension.clone(),
            width: f.width,
            height: f.height,
            size: f.size,
        }
    }

    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, File), MysqlError> {
        conn.drop_exec(
            "INSERT INTO `files` (`uuid`, `post`, `name`, `extension`, `width`, `height`, `size`) VALUES (:uuid, :post, :name, :extension, :width, :height, :size)",
            params! {
                "uuid" => self.uuid,
                "name" => &self.name,
                "extension" => self.extension.as_str(),
                "width" => self.width,
                "height" => self.height,
                "size" => self.size,
            },
        )
        .await?
        .first_exec(
            "SELECT * FROM `files` WHERE ID=(SELECT LAST_INSERT_ID())",
            (),
        )
        .await
        .map(|(c, v)| (c, v.unwrap()))
    }
}

impl File {
    pub async fn related_to_any_post_id(post_ids: &[u64], conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        if post_ids.is_empty() {
            return Ok((conn, Vec::default()))
        }
        let mut ids_str = String::new();
        for id in post_ids {
            ids_str.push_str(&id.to_string());
            ids_str.push(',');
        }
        ids_str.pop();
        conn.prep_exec(
            ["SELECT * FROM `files` WHERE `post` IN (", &ids_str, ")"].concat(),
            ()
        )
        .await?
        .map_and_drop(FromRow::from_row)
        .await
    }

    pub async fn delete_related_to_any_post_id(post_ids: &[u64], conn: Conn) -> Result<Conn, MysqlError> {
        if post_ids.is_empty() {
            return Ok(conn)
        }
        let mut ids_str = String::new();
        for id in post_ids {
            ids_str.push_str(&id.to_string());
            ids_str.push(',');
        }
        ids_str.pop();
        conn.drop_exec(
            ["DELETE FROM `files` WHERE `post` IN (", &ids_str, ")"].concat(),
            ()
        )
        .await
    }

    pub async fn delete_related_to_post_id(post_id: u64, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "DELETE FROM `files` WHERE `post`=:post",
            params! {
                "post" => post_id
            },
        )
        .await
    }

    pub async fn related_to_post_id(
        post_id: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec(
            "SELECT * FROM `files` WHERE `post`=:post",
            params! {
                "post" => post_id
            },
        )
        .await?
        .map_and_drop(Self::from_row)
        .await
    }

    pub async fn all_for_posts<I>(mut posts: I, conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError>
    where
        I: Iterator<Item = u64> + ExactSizeIterator,
    {
        let first = match posts.next() {
            Some(v) => v,

            None => return Ok((conn, Vec::new())),
        };

        // calculate first number digits count using logarithm hack
        let digits = (first as f64).log10() as u64;

        // prepare query string
        let mut digits_str = String::with_capacity((digits as usize + 1) * posts.len());

        // bind values
        digits_str.push_str(&first.to_string());
        digits_str.push(',');
        for id in posts {
            digits_str.push_str(&id.to_string());
            digits_str.push(',');
        }

        digits_str.pop();

        let query_str = [
            "SELECT * FROM `files` WHERE `post` IN (",
            &digits_str,
            ") ORDER BY `post` DESC",
        ]
        .concat();

        // execute
        conn.prep_exec(query_str, ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }

    /* dynamic */
    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `files`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
}

#[async_trait]
impl InsertMultiple<FileInfo> for (u64, &Vec<FileInfo>) {
    async fn commit(self, conn: Conn) -> Result<Conn, MysqlError> {
        let post = self.0;
        let params = self
            .1
            .iter()
            .map(|obj| {
                params! {
                    "uuid" => &obj.uuid,
                    "post" => post,
                    "name" => obj.name.as_ref(),
                    "extension" => &obj.extension.as_str(),
                    "width" => &obj.width,
                    "height" => &obj.height,
                    "size" => &obj.size,
                }
            })
            .collect::<Vec<_>>(); // FIXME

        let conn = conn.batch_exec("INSERT INTO `files` (`uuid`, `post`, `name`, `extension`, `width`, `height`, `size`) VALUES (:uuid, :post, :name, :extension, :width, :height, :size)", params).await?;

        Ok(conn)
    }
}
