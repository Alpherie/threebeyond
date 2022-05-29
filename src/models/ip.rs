use super::*;
use std::net::Ipv4Addr;

#[derive(Deserialize, Serialize, Debug)]
pub struct Ip {
    pub id: u64,
    pub addr: IpAddr,
    pub last: NaiveDateTime,
    pub first: NaiveDateTime,
}

pub struct IIp {
    pub addr: IpAddr,
}

impl IIp {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Ip), MysqlError> {
        let ip_str = self.addr.to_string();

        let conn = conn
            .drop_exec(
                "INSERT INTO `ips` (addr) VALUES (INET6_ATON(:addr))",
                params! {
                    "addr" => &ip_str
                },
            )
            .await?;

        conn.first_exec("SELECT `id`, INET6_NTOA(`addr`) as `addr`, `last`, `first` FROM `ips` WHERE addr=INET6_ATON(:ip)", params!{ "ip" => ip_str })
        .await
        .map(|(c, v)| (c, Ip::from_row(v.unwrap())))
    }
}

impl actix_web::FromRequest for Ip {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let connection_info = req.connection_info();

        //in future: realip_remote_addr
        let realip = connection_info.remote();
        let mut ip = match realip {
            Some(v) => {
                match v.parse() {
                    Ok(t) => t,
                    Err(_) => {
                        req.peer_addr().unwrap().ip()
                    }
                }
            },

            None => {
                req.peer_addr().unwrap().ip()
            }
        };

        let extensions = req.extensions();
        let token = extensions.get::<Option<models::Token>>().map(Clone::clone);
        drop(extensions);

        Box::pin(
            web::Data::<Pool>::extract(req)
                .map_err(ApiError::from)
                .and_then(move |pool| {
                    async move {
                        let mut conn = pool.get_conn().await?;
                        if let Some(Some(mut t)) = token {
                            let (nc, has) = t.has_perm("shadow_ip", conn).await?;
                            conn = nc;
                            if has {
                                ip = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
                            }
                        }

                        let (conn, record) = models::Ip::find_by_addr_updated(ip, conn).await?;

                        let record = match record {
                            Some(v) => v,

                            None => models::IIp { addr: ip }.create(conn).await?.1,
                        };

                        Ok::<_, ApiError>(record)
                    }
                })
                .boxed_local(),
        )
    }
}

impl Ip {
    /* static */
    pub async fn find_by_addr(
        addr: IpAddr,
        conn: Conn,
    ) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec("SELECT `id`, INET6_NTOA(`addr`) as `addr`, `last`, `first` FROM `ips` WHERE addr=INET6_ATON(:addr)", params!{ "addr" => addr.to_string() })
        .await
    }

    pub async fn find(
        id: u64,
        conn: Conn,
    ) -> Result<(Conn, Option<Self>), MysqlError> {
        conn.first_exec("SELECT `id`, INET6_NTOA(`addr`) as `addr`, `last`, `first` FROM `ips` WHERE id=:id", params!{ "id" => id })
        .await
    }

    pub async fn find_by_addr_updated(
        addr: IpAddr,
        conn: Conn,
    ) -> Result<(Conn, Option<Self>), MysqlError> {
        let (mut conn, ip) = Self::find_by_addr(addr, conn).await?;

        if let Some(ip) = &ip {
            conn = conn
                .drop_exec(
                    "UPDATE `ips` SET `last`=CURRENT_TIMESTAMP WHERE `id`=:id",
                    params! { "id" => ip.id },
                )
                .await?;
        }

        Ok((conn, ip))
    }

    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec(
            "SELECT `id`, INET6_ATON(`addr`) as `addr`, `last`, `first` FROM `ips`",
            (),
        )
        .await?
        .map_and_drop(Self::from_row)
        .await
    }
}

impl FromRow for Ip {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(Self {
            id: row.get("id").unwrap(),
            addr: row
                .get::<String, _>("addr")
                .unwrap()
                .parse()
                .expect("Failed to parse addr"),
            last: row.get("last").unwrap(),
            first: row.get("first").unwrap(),
        })
    }
}
