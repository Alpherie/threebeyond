use super::*;

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct Lang {
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

pub struct ILang<'a> {
    pub code: &'a str,
    pub name: &'a str,
}

impl Lang {
    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `langs`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }

    pub async fn find_by_code(code: &str, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec("SELECT * FROM `langs` WHERE `code`=:code", params! { "code" => code }).await
    }
}

impl ILang<'_> {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Lang), MysqlError> {
        let conn = conn.drop_exec("INSERT INTO `langs` (code, name) VALUES (:code, :name)",
            params! {
                "code"  => &self.code,
                "name"  => &self.name,
            }).await?;

        conn.first_exec(
            "SELECT * FROM `langs` WHERE code=:code",
            params! { "code" => self.code },
        )
        .await
        .map(|(c, v)| (c, v.unwrap()))
    }
}
