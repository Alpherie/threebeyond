use super::*;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, IpAddr};

#[derive(Serialize, Debug, Clone)]
pub enum BanIpAddress {
    V4(u16, Option<u16>),
    V6(u64, Option<u64>),
}

#[derive(Serialize, Debug, Clone)]
pub struct Ban {
    pub uuid: Uuid,
    pub lang: Option<String>,
    pub board: Option<u64>,
    pub because: Option<u64>,
    pub reason: u64,
    pub comment: String,
    pub ip: BanIpAddress,
    pub lasts: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub struct IBan<'a> {
    pub lang: Option<&'a str>,
    pub board: Option<u64>,
    pub because: Option<u64>,
    pub reason: u64,
    pub comment: &'a str,
    pub ip: BanIpAddress,
    pub minutes: u32,
}

impl From<SocketAddr> for BanIpAddress {
    fn from(t: SocketAddr) -> Self {
        match t {
            SocketAddr::V4(addr) => {
                let o = addr.ip().octets();
                let subnet: u16 = ((o[0] as u16) << 8) + o[1] as u16;
                let device: u16 = ((o[2] as u16) << 8) + o[3] as u16;
                Self::V4(subnet, Some(device))
            },

            SocketAddr::V6(addr) => {
                let o = addr.ip().octets();
                let subnet: u64 = ((((((((((((((o[0] as u64) << 8) + o[1] as u64) << 8) + o[2] as u64) << 8) + o[3] as u64) << 8) + o[4] as u64) << 8) + o[5] as u64) << 8 )+ o[6] as u64) << 8) + o[7] as u64;
                let device: u64 = ((((((((((((((o[8] as u64) << 8) + o[9] as u64) << 8) + o[10] as u64) << 8) + o[11] as u64) << 8) + o[12] as u64) << 8) + o[13] as u64) << 8) + o[14] as u64) << 8) + o[15] as u64;
                Self::V6(subnet, Some(device))
            },
        }
    }
}

impl From<IpAddr> for BanIpAddress {
    fn from(t: IpAddr) -> Self {
        match t {
            IpAddr::V4(addr) => {
                let o = addr.octets();
                let subnet: u16 = ((o[0] as u16) << 8) + o[1] as u16;
                let device: u16 = ((o[2] as u16) << 8) + o[3] as u16;
                Self::V4(subnet, Some(device))
            },

            IpAddr::V6(addr) => {
                let o = addr.octets();
                let subnet: u64 = ((((((((((((((o[0] as u64) << 8) + o[1] as u64) << 8) + o[2] as u64) << 8) + o[3] as u64) << 8) + o[4] as u64) << 8) + o[5] as u64) << 8 )+ o[6] as u64) << 8) + o[7] as u64;
                let device: u64 = ((((((((((((((o[8] as u64) << 8) + o[9] as u64) << 8) + o[10] as u64) << 8) + o[11] as u64) << 8) + o[12] as u64) << 8) + o[13] as u64) << 8) + o[14] as u64) << 8) + o[15] as u64;
                Self::V6(subnet, Some(device))
            },
        }
    }
}

impl Into<IpAddr> for BanIpAddress {
    fn into(self) -> IpAddr {
        match self {
            BanIpAddress::V4(subnet, device) => {
                let subnet = subnet.to_be_bytes();
                let device = device.unwrap_or(0).to_be_bytes();
                IpAddr::V4(Ipv4Addr::new(subnet[0], subnet[1], device[0], device[1]))
            },

            BanIpAddress::V6(subnet, device) => {
                let subnet = subnet.to_be_bytes();
                let device = device.unwrap_or(0).to_be_bytes();
                IpAddr::V6(Ipv6Addr::new(
                    u16::from_be_bytes([subnet[0], subnet[1]]),
                    u16::from_be_bytes([subnet[2], subnet[3]]),
                    u16::from_be_bytes([subnet[4], subnet[5]]),
                    u16::from_be_bytes([subnet[6], subnet[7]]),
                    u16::from_be_bytes([device[0], device[1]]),
                    u16::from_be_bytes([device[2], device[3]]),
                    u16::from_be_bytes([device[4], device[5]]),
                    u16::from_be_bytes([device[6], device[7]]),
                ))
            },
        }
    }
}

impl Ban {
    pub fn from_row_ipv4(row: Row) -> Self {
        Self {
            uuid: row.get("uuid").unwrap(),
            lang: row.get("lang").unwrap(),
            board: row.get("board").unwrap(),
            because: row.get("because").unwrap(),
            reason: row.get("reason").unwrap(),
            comment: row.get("comment").unwrap(),
            ip: BanIpAddress::V4(row.get("subnet").unwrap(), row.get("device").unwrap()),
            lasts: row.get("lasts").unwrap(),
            created_at: row.get("created_at").unwrap(),
        }
    }

    pub fn from_row_ipv6(row: Row) -> Self {
        Self {
            uuid: row.get("uuid").unwrap(),
            lang: row.get("lang").unwrap(),
            board: row.get("board").unwrap(),
            because: row.get("because").unwrap(),
            reason: row.get("reason").unwrap(),
            comment: row.get("comment").unwrap(),
            ip: BanIpAddress::V6(row.get("subnet").unwrap(), row.get("device").unwrap()),
            lasts: row.get("lasts").unwrap(),
            created_at: row.get("created_at").unwrap(),
        }
    }

    pub async fn find_wn_lang_wn_board_first(addr: BanIpAddress, lang: &str, board: &str, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        match addr {
            BanIpAddress::V4(subnet, device) => {
                conn.first_exec(
                    "SELECT `ip4_bans`.* FROM `ip4_bans` INNER JOIN `boards` ON `boards`.`short`=:board AND `boards`.`lang`=:lang WHERE `ip4_bans`.`lasts` > CURRENT_TIMESTAMP AND `ip4_bans`.`subnet`=:subnet AND (`ip4_bans`.`lang` IS NULL OR `ip4_bans`.`lang`=:lang) AND (`ip4_bans`.`board` IS NULL OR `ip4_bans`.`board`=`boards`.`id`) AND (`ip4_bans`.`device` IS NULL OR `ip4_bans`.`device`=:device)",
                    params! { "subnet" => subnet, "device" => device, "lang" => lang, "board" => board },
                )
                .await
                .map(|(c, ov)| (c, ov.map(Self::from_row_ipv4)))
            },
            BanIpAddress::V6(subnet, device) => {
                conn.first_exec(
                    "SELECT `ip6_bans`.* FROM `ip6_bans` INNER JOIN `boards` ON `boards`.`short`=:board AND `boards`.`lang`=:lang WHERE `ip4_bans`.`lasts` > CURRENT_TIMESTAMP AND `ip4_bans`.`subnet`=:subnet AND (`ip6_bans`.`lang` IS NULL OR `ip6_bans`.`lang`=:lang) AND (`ip6_bans`.`board` IS NULL OR `ip6_bans`.`board`=`boards`.`id`) AND (`ip6_bans`.`device` IS NULL OR `ip6_bans`.`device`=:device)",
                    params! { "subnet" => subnet, "device" => device, "lang" => lang, "board" => board },
                )
                .await
                .map(|(c, ov)| (c, ov.map(Self::from_row_ipv6)))
            }
        }
    }

    pub async fn find_on_service_first(addr: BanIpAddress, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
        match addr {
            BanIpAddress::V4(subnet, device) => {
                conn.first_exec(
                    "SELECT * FROM `ip4_bans` WHERE `subnet`=:subnet AND `board` IS NULL AND `lang` IS NULL AND (`device` IS NULL OR `device`=:device)",
                    params! { "subnet" => subnet, "device" => device },
                )
                .await
                .map(|(c, ov)| (c, ov.map(Self::from_row_ipv4)))
            },
            BanIpAddress::V6(subnet, device) => {
                conn.first_exec(
                    "SELECT * FROM `ip6_bans` WHERE `subnet`=:subnet AND `board` IS NULL AND `lang` IS NULL AND (`device` IS NULL OR `device`=:device)",
                    params! { "subnet" => subnet, "device" => device },
                )
                .await
                .map(|(c, ov)| (c, ov.map(Self::from_row_ipv6)))
            }
        }
    }

    pub async fn find_for_device_or_subnet_ipv4(subnet: u16, device: u16, conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec(
            "SELECT * FROM `ip4_bans` WHERE `subnet`=:subnet AND (`device` IS NULL OR `device`=:device)",
            params! { "subnet" => subnet, "device" => device },
        )
        .await?
        .map_and_drop(Self::from_row_ipv4)
        .await
    }
}

impl<'a> IBan<'a> {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Ban), MysqlError> {
        let uuid = Uuid::new_v4();
        let mut params = params! {
            "uuid" => &uuid,
            "lang" => self.lang,
            "board" => self.board,
            "because" => self.because,
            "reason" => self.reason,
            "comment" => self.comment,
            "minutes" => self.minutes
        };

        let (func, table): (& dyn Fn(Row) -> Ban, _) = match self.ip {
            BanIpAddress::V4(subnet, device) => {
                params.push((String::from("subnet"), subnet.into()));
                params.push((String::from("device"), device.into()));

                (&Ban::from_row_ipv4, "ip4_bans")
            },
            BanIpAddress::V6(subnet, device) => {
                params.push((String::from("subnet"), subnet.into()));
                params.push((String::from("device"), device.into()));
            
                (&Ban::from_row_ipv6, "ip6_bans")
            }
        };

        let conn = conn.drop_exec(
            ["INSERT INTO ", &table, " (uuid, subnet, device, lang, board, because, reason, comment, lasts) VALUES (:uuid, :subnet, :device, :lang, :board, :because, :reason, :comment, CURRENT_TIMESTAMP + INTERVAL :minutes MINUTE)"].concat(), 
            params
        ).await?;

        let (conn, row) = conn.first_exec(
            ["SELECT * FROM ", &table, " WHERE `uuid`=:uuid"].concat(),
            params! { "uuid" => uuid },
        )
        .await?;

        Ok((conn, func(row.unwrap())))
    }
}