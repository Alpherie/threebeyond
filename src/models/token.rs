use super::*;

/* TO DO */
#[derive(FromRow, Debug, Clone)]
pub struct Token {
    pub id: u64,
    pub uuid: Uuid,
    pub is_mod: bool,

    #[field_default]
    roles: Option<Vec<Role>>,
    #[field_default]
    perms: Option<Vec<Perm>>,

    pub at: NaiveDateTime,
}

pub struct IToken {
    pub uuid: Uuid,
    pub is_mod: bool,
}

impl IToken {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Token), MysqlError> {
        let conn = conn
            .drop_exec(
                "INSERT INTO `tokens` (uuid) VALUES (:uuid)",
                params! {
                    "uuid" => &self.uuid.as_u128().to_le_bytes(),
                    "is_mod" => self.is_mod
                },
            )
            .await?;

        let (conn, row): (_, Option<Row>) = conn
            .first_exec(
                "SELECT * FROM `tokens` WHERE ID=(SELECT LAST_INSERT_ID())",
                (),
            )
            .await?;

        Ok((conn, Token::from_row(row.unwrap())))
    }

    /* static */
    pub async fn random(conn: Conn) -> Result<(Conn, Token), MysqlError> {
        Self {
            is_mod: false,
            uuid: Uuid::new_v4(),
        }
        .create(conn)
        .await
    }
}

// FIXME: REMOVE, LEGACY CODE
impl actix_web::FromRequest for Token {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // FIXME: govnokod, should be removed
        match req.extensions().get::<Option<Self>>().map(Clone::clone) {
            Some(Some(v)) => Box::pin(async { Ok(v) }),
            None | Some(None) => {
                Box::pin(async {
                    Err(ApiError::authorization_required())
                })
                // let header = req.headers().get("Authorization").map(Clone::clone);

                // Box::pin(
                //     web::Data::<Pool>::extract(req)
                //         .map_err(ApiError::from)
                //         .and_then(move |pool| {
                //             async move {
                //                 let header = header.ok_or(ApiError::authorization_required())?;

                //                 let auth = header.to_str()?;

                //                 let uuid = Uuid::parse_str(auth)
                //                     .map_err(|_| ApiError::authorization_bad_token())?;

                //                 let (_, token) = Self::find(&uuid, pool.get_conn().await?).await?;

                //                 let token = token.ok_or(ApiError::authorization_bad_token())?;

                //                 Ok(token)
                //             }
                //         })
                //         .boxed_local()
                // )
            }
        }
    }
}

impl Token {
    /* dynamic */
    pub async fn remove_role(&self, role: &models::Role, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "DELETE FROM `token_roles` WHERE `token`=:id AND `role`=:role",
            params! {"id" => self.id, "role" => role.id},
        )
        .await
    }

    pub async fn delete(&self, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec(
            "DELETE FROM `tokens` WHERE `id`=:id",
            params! {"id" => self.id},
        )
        .await?
        .drop_exec(
            "DELETE FROM `token_roles` WHERE `token`=:id",
            params! {"id" => self.id},
        )
        .await
    }

    pub async fn has_perm(&mut self, perm: &str, conn: Conn) -> Result<(Conn, bool), MysqlError> {
        let (conn, perms) = self.perms(conn).await?;

        Ok((conn, perms.iter().find(|p| p.name == perm).is_some()))
    }

    pub async fn has_role(&mut self, role: &str, conn: Conn) -> Result<(Conn, bool), MysqlError> {
        let (conn, roles) = self.roles(conn).await?;

        Ok((conn, roles.iter().find(|r| r.name == role).is_some()))
    }

    // pub async fn has_roles<'a, V: Into<Vec<&'a str>>>(
    //     &'a mut self,
    //     remain: V,
    //     conn: Conn,
    // ) -> Result<(Conn, (bool, Option<Vec<&str>>)), MysqlError> {
    //     let (conn, roles) = self.roles(conn).await?;

    //     // TODO: maybe should use hashmap
    //     let mut missing = remain.into();

    //     for role in roles {
    //         match missing.iter().position(|r| r == &role.name) {
    //             Some(index) => {
    //                 missing.remove(index);
    //                 ()
    //             }
    //             None => (),
    //         };
    //     }

    //     Ok((
    //         conn,
    //         if missing.len() == 0 {
    //             (true, None)
    //         } else {
    //             (false, Some(missing))
    //         },
    //     ))
    // }

    async fn get_roles(&self, conn: Conn) -> Result<(Conn, Vec<Role>), MysqlError> {
        models::Role::related_to_token(self, conn).await
    }

    pub async fn roles<'a>(
        &'a mut self,
        mut conn: Conn,
    ) -> Result<(Conn, &'a Vec<Role>), MysqlError> {
        if let None = self.roles {
            let (c, roles) = self.get_roles(conn).await?;

            self.roles = Some(roles);
            conn = c;
        };

        Ok((conn, self.roles.as_ref().unwrap()))
    }

    pub async fn update_roles<'a>(
        &'a mut self,
        mut conn: Conn,
    ) -> Result<(Conn, &'a Vec<Role>), MysqlError> {
        let (c, roles) = self.get_roles(conn).await?;

        self.roles = Some(roles);
        conn = c;

        Ok((conn, self.roles.as_ref().unwrap()))
    }

    pub async fn get_perms(&self, conn: Conn) -> Result<(Conn, Vec<Perm>), MysqlError> {
        Perm::related_to_token(self.id, conn).await
    }

    pub async fn perms<'a>(
        &'a mut self,
        mut conn: Conn,
    ) -> Result<(Conn, &'a Vec<Perm>), MysqlError> {
        if let None = self.perms {
            let (c, perms) = self.get_perms(conn).await?;

            self.perms = Some(perms);
            conn = c;
        };

        Ok((conn, self.perms.as_ref().unwrap()))
    }

    pub async fn update_perms<'a>(
        &'a mut self,
        mut conn: Conn,
    ) -> Result<(Conn, &'a Vec<Perm>), MysqlError> {
        let (c, perms) = self.get_perms(conn).await?;

        self.perms = Some(perms);
        conn = c;

        Ok((conn, self.perms.as_ref().unwrap()))
    }

    pub async fn assign(&self, role: &models::Role, conn: Conn) -> Result<Conn, MysqlError> {
        conn.drop_exec("INSERT IGNORE INTO `token_roles` SELECT :token,:role WHERE NOT EXISTS(SELECT 1 FROM `token_roles` WHERE (`token`=:token AND `role`=:role))", params!{
            "token" => self.id,
            "role"  => role.id
        }).await
    }

    /* static */
    pub async fn from_request_optional(
        req: &HttpRequest,
        conn: Conn,
    ) -> Result<(Conn, Option<Token>), MysqlError> {
        let headers = req.headers();

        let auth = match headers.get("Authorization") {
            Some(v) => v,
            None => return Ok((conn, None)),
        }
        .to_str()
        .expect("Failed to get authorization token"); // FIXME FIXME FIXMe

        let uuid = match Uuid::parse_str(auth) {
            Ok(v) => v,
            Err(_) => return Ok((conn, None)),
        };

        Token::find_by_uuid(&uuid, conn).await
    }

    pub async fn from_request(req: &HttpRequest, conn: Conn) -> Result<(Conn, Token), ApiError> {
        let headers = req.headers();

        let header = headers
            .get("Authorization")
            .ok_or(ApiError::authorization_required())?;

        let auth = header.to_str()?;

        let uuid = Uuid::parse_str(auth).map_err(|_| ApiError::authorization_bad_token())?;

        let (conn, token) = Self::find_by_uuid(&uuid, conn).await?;
        let token = token.ok_or(ApiError::authorization_bad_token())?;

        Ok((conn, token))
    }

    pub async fn from_request_role<S: AsRef<str>>(
        req: &HttpRequest,
        role: S,
        conn: Conn,
    ) -> Result<(Conn, Self), ApiError> {
        let (conn, mut token) = Self::from_request(req, conn).await?;
        let (conn, allowed) = token.has_role(role.as_ref(), conn).await?;

        if !allowed {
            Err(ApiError::access_missing_role(role.as_ref()))
        } else {
            Ok((conn, token))
        }
    }

    pub async fn from_request_perm<S: AsRef<str>>(
        req: &HttpRequest,
        perm: S,
        conn: Conn,
    ) -> Result<(Conn, Self), ApiError> {
        let (conn, mut token) = Self::from_request(req, conn).await?;
        let (conn, allowed) = token.has_perm(perm.as_ref(), conn).await?;

        if !allowed {
            Err(ApiError::access_missing_perm(perm.as_ref()))
        } else {
            Ok((conn, token))
        }
    }

    pub async fn find_by_uuid(uuid: &Uuid, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec(
            "SELECT * FROM `tokens` WHERE `uuid`=:uuid",
            params! { "uuid" => uuid.as_bytes() },
        )
        .await
        .map(|(c, v)| (c, v.map(Self::from_row)))
    }

    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `tokens`", ())
            .await?
            .map_and_drop(|row| Self::from_row(row))
            .await
    }
}

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Token", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("uuid", &self.uuid.to_hyphenated().to_string())?;
        state.serialize_field(
            "roles",
            &match &self.roles {
                Some(v) => v.iter().map(|v| v.name.as_str()).collect::<Vec<_>>(),
                None => Vec::new(),
            },
        )?;
        state.serialize_field(
            "perms",
            &match &self.perms {
                Some(v) => v.iter().map(|v| v.name.as_str()).collect::<Vec<_>>(),
                None => Vec::new(),
            },
        )?;

        state.end()
    }
}
