use super::*;

pub mod auth;
pub mod ban_reasons;
pub mod captcha;
pub mod counting;
pub mod donations;
pub mod langs;
pub mod system;

use threebeyond_codegen_auth as auth_gen;
use threebeyond_codegen_response as res;

#[get("stats")]
pub async fn stats_json(
    pool: Data<Pool>,
    state: Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get_conn().await?;

    res::json!(state.stats(conn).await?.1)
}

#[get("ip.json")]
pub async fn ip_info(req: HttpRequest, ip: models::Ip) -> Result<impl Responder, ApiError> {
    let connection_info = req.connection_info();

    //in future: realip_remote_addr
    let realip = connection_info.remote();
    println!("{:?}", realip);
    res::json!(ip)
}

// #[get("test")]
// pub async fn test_ban(pool: web::Data<Pool>, req: HttpRequest) -> Result<impl Responder, ApiError> {
//     let addr = req.peer_addr().unwrap();
//     let conn = pool.get_conn().await?;

//     match addr {
//         std::net::SocketAddr::V4(a) => {
//             let o = a.ip().octets();
//             let subnet: u16 = (o[0] as u16) << 8 << o[1];
//             let device: u16 = (o[2] as u16) << 8 << o[3];

//             // find
//             let (conn, ban) =
//                 models::Ban::find_for_device_or_subnet_ipv4(subnet, device, conn).await?;
//             println!("{:?}", ban);

//             let iban = models::IBan {
//                 because: None,
//                 comment: "www",
//                 reason: 0,
//                 lang: None,
//                 board: None,
//                 ip: models::BanIpAddress::V4(subnet, Some(device)),
//                 lasts: time_now() + chrono::Duration::weeks(6),
//             };

//             println!("{:?}", iban.create(conn).await?.1);
//         }
//         _ => panic!("w"),
//     };

//     res::no!()
// }
