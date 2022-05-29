use super::*;

/* TO DO */
#[derive(FromRow, Serialize, Debug, Clone)]
pub struct Role {
    pub id: u64,
    pub name: String,
}

pub struct IRole<'a> {
    pub name: &'a str,
}

impl<'a> IRole<'a> {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Role), MysqlError> {
        conn.drop_exec(
            "INSERT INTO `roles` (name) VALUES (:name)",
            params! {
                "name" => &self.name,
            },
        )
        .await?
        .first_exec(
            "SELECT * FROM `roles` WHERE ID=(SELECT LAST_INSERT_ID())",
            (),
        )
        .await
        .map(|(c, v)| (c, Role::from_row(v.unwrap())))
    }
}

impl Role {
    /* dynamic */
    pub async fn remove_perm(
        &self,
        perm: &models::Perm,
        conn: Conn,
    ) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "DELETE FROM `role_perms` WHERE `role`=:role AND `perm`=:perm",
            params! { "role" => self.id, "perm" => perm.id },
        )
        .await
    }

    pub async fn assign(&self, perm: &models::Perm, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec("INSERT IGNORE INTO `role_perms` SELECT :role,:perm WHERE NOT EXISTS(SELECT 1 FROM `role_perms` WHERE (`role`=:role AND `perm`=:perm))", params!{
            "role" => self.id,
            "perm" => perm.id
        }).await
    }

    pub async fn assign_to(&self, token: u32, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec("INSERT IGNORE INTO `token_roles` SELECT :token,:role WHERE NOT EXISTS(SELECT 1 FROM `token_roles` WHERE (`token`=:token AND `role`=:role))", params!{
            "token" => token,
            "role"  => self.id
        }).await
    }

    /* static */
    pub async fn related_to_token(
        token: &models::Token,
        conn: Conn,
    ) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT `roles`.* FROM (SELECT * FROM `token_roles` WHERE `token`=:token) `rels` INNER JOIN `roles` WHERE `roles`.`id`=`rels`.`role`", params! {
            "token" => token.id
        }).await?
        .map_and_drop(Self::from_row)
        .await
    }

    pub async fn any(name: &str, conn: Conn) -> Result<(Conn, Self), MysqlError> {
        let (conn, r) = Self::find_by_name(name, conn).await?;

        Ok(match r {
            Some(v) => (conn, v),
            None => IRole { name: name }.create(conn).await?,
        })
    }

    pub async fn find_by_name(name: &str, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec(
            "SELECT * FROM `roles` WHERE `name`=:name",
            params! { "name" => name },
        )
        .await
        .map(|(c, v)| (c, v.map(Self::from_row)))
    }

    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `roles`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
}
