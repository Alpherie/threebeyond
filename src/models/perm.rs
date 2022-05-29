use super::*;

/* TO DO */
#[derive(FromRow, Serialize, Debug, Clone)]
pub struct Perm {
    pub id: u64,
    pub name: String,
}

pub struct IPerm<'a> {
    pub name: &'a str,
}

impl<'a> IPerm<'a> {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Perm), MysqlError> {
        let conn = conn
            .drop_exec(
                "INSERT INTO `perms` (name) VALUES (:name)",
                params! {
                    "name" => &self.name,
                },
            )
            .await?;

        conn.first_exec(
            "SELECT * FROM `perms` WHERE ID=(SELECT LAST_INSERT_ID())",
            (),
        )
        .await
        .map(|(c, v)| (c, Perm::from_row(v.unwrap())))
    }
}

impl Perm {
    /* dynamic */
    pub async fn assign_to_role(&self, role_id: u64, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "INSERT INTO `role_perms` (`role`, `perm`) VALUES(:role, :perm)",
            params! {
                "role" => role_id,
                "perm" => self.id
            },
        )
        .await
    }

    /* static */
    pub async fn related_to_role(
        role_id: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec(
            "SELECT `perms`.*
                FROM `role_perms`
            INNER JOIN `perms` ON `perms`.`id`=`role_perms`.`perm`
                WHERE `role_perms`.`role`=:role_id
        ",
            params! {
                "role_id" => role_id
            },
        )
        .await?
        .map_and_drop(Self::from_row)
        .await
    }

    pub async fn related_to_token(
        token_id: u64,
        conn: Conn,
    ) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec(
            "
            SELECT `perms`.* 
                FROM (
                    SELECT DISTINCT `perm`
                        FROM (
                            SELECT *
                                FROM `token_roles` 
                                    WHERE `token`=:token ) `roles`
                                INNER JOIN `role_perms` ON `role_perms`.`role`=`roles`.`role`
                        ) `p`
            INNER JOIN `perms` ON `perms`.`id`=`p`.`perm`
        ",
            params! {
                    "token" => token_id
            },
        )
        .await?
        .map_and_drop(Self::from_row)
        .await
    }

    pub async fn any(name: &str, conn: Conn) -> Result<(Conn, Self), MysqlError> {
        let (conn, r) = Self::find_by_name(name, conn).await?;

        Ok(match r {
            Some(v) => (conn, v),
            None => IPerm { name: name }.create(conn).await?,
        })
    }

    pub async fn find_by_name(name: &str, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        let (conn, row): (_, Option<Row>) = conn
            .first_exec(
                "SELECT * FROM `perms` WHERE `name`=:name",
                params! { "name" => name },
            )
            .await?;

        Ok((conn, row.map(Self::from_row)))
    }

    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `perms`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
}
