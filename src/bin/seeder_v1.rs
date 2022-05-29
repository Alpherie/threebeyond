use clap::{App, Arg};
use std::error::Error;
use threebeyond::{perm_rep::*, *};

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("3beyond perm-roles-seeder")
        .author("じぜロ")
        .about("Part of the ですかちゃん project")
        .arg(
            Arg::with_name("db")
                .long("database_url")
                .short("d")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let db = matches.value_of("db").unwrap();
    let pool = mysql_async::Pool::new(db);
    let mut conn = pool.get_conn().await?;

    const langs: &[(
        &str,
        &str,
        &[(&str, &str, &str, &str, u64, u64, u64, u32, bool)],
    )] = &[
        // ("en", "English", &[
        // 	("s", "Service", "Service development & Suggestion", 10, 10, 5, false),
        // 	("a", "Anime/Random", "Anime-random stuff", 10, 10, 5, false),
        // 	("b", "Other/Random", "Random stuff", 10, 10, 5, false),
        // 	("v", "Videogames", "games for idiots", 10, 10, 5, false),
        // 	("o", "Otaku/Bullsiht", "Otaku-random stuff", 10, 10, 5, false),
        // ]),
        (
            "ru",
            "Русский",
            &[
                (
                    "a",
                    "Аниме",
                    "Китайские мультики",
                    "",
                    10,
                    10,
                    5,
                    200,
                    false,
                ),
                ("b", "Бред", "Битардск", "", 10, 10, 5, 200, false),
                // ("v", "Видеоигры", "Доточки и ксочки разной степени.", 10, 10, 5, false),
                (
                    "serv",
                    "Сервис",
                    "Обсуждение и развитие сервиса",
                    "",
                    10,
                    10,
                    5,
                    200,
                    true,
                ),
            ],
        ),
        // (
        //     "en",
        //     "English",
        //     &[
        //         ("a", "Anime/Random", "Random cartoon stuff", 10, 10, 5, false),
        //         ("b", "Other/Random", "Random stuff", 10, 10, 5, false),
        //         // ("v", "Видеоигры", "Доточки и ксочки разной степени.", 10, 10, 5, false),
        //     ],
        // ),
    ];

    const ban_reasons: &[&str] = &["CP", "OTHER", "SPAM", "WIPE"];

    const additional_root_roles: &[&str] = &["reasons::create", "shadow_ip"];

    // go
    for reason in ban_reasons {
        conn = models::IBanReason { alias: reason }.create(conn).await?.0;
    }
    println!("{:?}", "Ban reasons -- done");

    let (nc, role_root) = models::IRole { name: "root" }.create(conn).await?;
    conn = nc;
    for permn in additional_root_roles {
        let (nc, perm) = models::Perm::any(permn, conn).await?;
        conn = nc;
        conn = role_root.assign(&perm, conn).await?;
    }

    let (mut conn, root) = models::IToken::random(conn).await?;

    conn = root.assign(&role_root, conn).await?;

    println!("{:?}", "Root base -- done");

    for lang in langs {
        conn = models::ILang {
            code: lang.0,
            name: lang.1,
        }
        .create(conn)
        .await?
        .0;

        let (mut nc, operator) =
            models::Role::any(&["langs::", &lang.0, "::operator"].concat(), conn).await?;
        for perm in perm_rep::general_lang_perms {
            let (nc_, perm) =
                models::Perm::any(&["langs::", &lang.0, "::", perm].concat(), nc).await?;
            nc = operator.assign(&perm, nc_).await?;
        }
        conn = root.assign(&operator, nc).await?;

        // for board in lang.2 {
        //     conn = models::IBoard {
        //         lang: lang.0,
        //         short: board.0,
        //         name: board.1,
        //         description: board.2,
        //         rules: board.3,
        //         pages_count: board.4,
        //         per_page: board.5,
        //         last_replies: board.6,
        //         bumplimit: board.7,
        //         thread_creating_limited: if board.8 { 1 } else { 0 },
        //     }
        //     .create(conn)
        //     .await?
        //     .0;

        //     let base = ["langs::", lang.0, "::boards::", board.0, "::"].concat();
        //     let (nc, moder) = models::IRole {
        //         name: &[&base, "moder"].concat(),
        //     }
        //     .create(conn)
        //     .await?;
        //     let (nc, admin) = models::IRole {
        //         name: &[&base, "admin"].concat(),
        //     }
        //     .create(nc)
        //     .await?;
        //     conn = nc;

        //     for perm in general_board_perms {
        //         let perm_dir = [&base, *perm].concat();
        //         let (nc, perm) = models::Perm::any(&perm_dir, conn).await?;
        //         conn = moder.assign(&perm, nc).await?;
        //     }

        //     for perm in admin_board_perms {
        //         let perm_dir = [&base, *perm].concat();
        //         let (nc, perm) = models::Perm::any(&perm_dir, conn).await?;
        //         conn = admin.assign(&perm, nc).await?;
        //     }

        //     // add to root
        //     conn = root.assign(&moder, conn).await?;
        //     conn = root.assign(&admin, conn).await?;
        // }
    }

    println!("{:?}", "Lang&Boards -- done");

    println!("ROOT TOKEN: {:?}", root.uuid);

    Ok(())
}
