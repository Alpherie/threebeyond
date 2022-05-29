use super::*;

#[derive(Serialize, Debug, Clone, FromRow)]
pub struct TokenBan {
    pub uuid: Uuid,
    pub token: u64,
    pub lang: Option<String>,
    pub board: Option<u64>,
    pub because: Option<u64>,
    pub reason: u64,
    pub comment: String,
    pub lasts: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub struct ITokenBan<'a> {
    pub token: u64,
    pub lang: Option<&'a str>,
    pub board: Option<u64>,
    pub because: Option<u64>,
    pub reason: u64,
    pub comment: &'a str,
    pub minutes: u32,
}

impl TokenBan {
	pub async fn find_by_token_id(token_id: u64, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
		conn.first_exec(
			"SELECT * FROM `token_bans` WHERE `token`=:token_id",
			params! { "token_id" => token_id }
		)
		.await
	}
}

impl<'a> ITokenBan<'a> {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, TokenBan), MysqlError> {
        let uuid = Uuid::new_v4();
        let params = params! {
            "uuid" => &uuid,
            "token" => self.token,
            "lang" => self.lang,
            "board" => self.board,
            "because" => self.because,
            "reason" => self.reason,
            "comment" => self.comment,
            "minutes" => self.minutes
        };

        let conn = conn.drop_exec(
            "INSERT INTO `token_bans` (uuid, token, lang, board, because, reason, comment, lasts) VALUES (:uuid, :token, :lang, :board, :because, :reason, :comment, CURRENT_TIMESTAMP + INTERVAL :minutes MINUTE)",
            params
        ).await?;

        let (conn, row) = conn.first_exec(
            "SELECT * FROM `token_bans` WHERE `uuid`=:uuid",
            params! { "uuid" => uuid },
        )
        .await?;

        Ok((conn, TokenBan::from_row(row.unwrap())))
    }
}