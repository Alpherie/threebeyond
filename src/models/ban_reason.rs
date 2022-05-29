use super::*;

#[derive(FromRow, Serialize, Debug, Clone)]
pub struct BanReason {
	pub id: u64,
	pub alias: String,
	pub created_at: NaiveDateTime
}

pub struct IBanReason<'a> {
	pub alias: &'a str
}

impl BanReason {
    pub async fn from_route_by_alias(
        alias: &str,
        conn: Conn,
    ) -> Result<(Conn, Self), ApiError> {
        match Self::find_by_alias(alias, conn).await? {
            (_conn, None) => Err(ApiError::ban_reason_not_found(alias)),

            (conn, Some(v)) => Ok((conn, v)),
        }
    }

	pub async fn find_by_alias(alias: &str, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
		conn.first_exec(
            "SELECT * FROM `ban_reasons` WHERE `alias`=:alias",
            params! { "alias" => alias },
        )
        .await
	}

    pub async fn find(id: u64, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec(
            "SELECT * FROM `ban_reasons` WHERE `id`=:id",
            params! { "id" => id },
        )
        .await
    }

	pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `ban_reasons`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
}

impl<'a> IBanReason<'a> {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, BanReason), MysqlError> {
        let conn = conn.drop_exec("INSERT INTO `ban_reasons` (alias) VALUES (:alias)",
            params! {
                "alias"	=> &self.alias,
            }).await?;

        conn.first_exec(
            "SELECT * FROM `ban_reasons` WHERE ID=(SELECT LAST_INSERT_ID())",
            (),
        )
        .await
        .map(|(c, v)| (c, v.unwrap()))
    }
}
