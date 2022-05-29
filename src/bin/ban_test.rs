use std::net::{IpAddr, Ipv4Addr};
use threebeyond::*;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let ip = models::BanIpAddress::V6(2281337, Some(1000));
    let pool = Data::new(mysql_async::Pool::new("mysql://root:root@localhost/beyond"));
    let conn = pool.get_conn().await?;

    // let a = IpAddr::V4(Ipv4Addr::new(176, 59, 133, 50));

    // let b: models::BanIpAddress = a.into();

    // println!("{:?}", b);

    let tokens = models::Token::all(conn).await?;

    println!("{:?}", tokens.1[0]);

    // let ban = models::IBan {
    //     ip,
    //     lang: None,
    //     board: None,
    //     because: None,
    //     reason: 0,
    //     comment: "Hello",
    //     lasts: time_now(),
    // };

    // println!("{:?}", ban.create(conn).await?.1);

    Ok(())
}
