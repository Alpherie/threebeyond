use clap::{App, Arg};
use std::error::Error;
use threebeyond::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("3beyond admin control CLI")
        .author("じぜロ")
        .about("Part of the ですかちゃん project")
        .arg(
            Arg::with_name("db")
                .long("database_url")
                .short("d")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("section")
                .index(1)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("command")
                .index(2)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("description")
                .long("description")
                .takes_value(true),
        )
        .arg(Arg::with_name("rules").long("rules").takes_value(true))
        .arg(Arg::with_name("pages").long("pages").takes_value(true))
        .arg(
            Arg::with_name("bumplimit")
                .long("bumplimit")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threads_per_page")
                .long("threads_per_page")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("last_replies")
                .long("last_replies")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("slowmode")
                .long("slowmode")
                .takes_value(true),
        )
        .arg(Arg::with_name("op-deletion").long("op-deletion"))
        .arg(Arg::with_name("op-oppost").long("op-oppost"))
        .arg(Arg::with_name("lang").long("lang").takes_value(true))
        .arg(Arg::with_name("short").long("short").takes_value(true))
        .arg(Arg::with_name("token").long("token").takes_value(true))
        .arg(Arg::with_name("role").long("role").takes_value(true))
        .arg(Arg::with_name("perm").long("perm").takes_value(true))
        .arg(Arg::with_name("name").long("name").takes_value(true))
        .arg(Arg::with_name("perms").long("perms").takes_value(true))
        .arg(
            Arg::with_name("per_page")
                .long("per_page")
                .takes_value(true),
        )
        .arg(Arg::with_name("roles").long("roles").takes_value(true))
        .get_matches();

    let db = matches.value_of("db").unwrap();
    let pool = mysql_async::Pool::new(db);
    let conn = pool.get_conn().await?;

    match matches.value_of("section").unwrap() {
        "board" => match matches.value_of("command").unwrap() {
            "new" => {
                let lang = matches.value_of("lang").expect("lang");
                let short = matches.value_of("short").expect("short");
                let name = matches.value_of("name").expect("name");
                let description = matches.value_of("description").expect("description");
                let slowmode = matches.value_of("slowmode").map(|x| x.parse().unwrap());
                let last_replies = matches
                    .value_of("last_replies")
                    .expect("last_replies")
                    .parse()
                    .unwrap();
                let rules = matches.value_of("rules").expect("rules");
                let pages_count = matches.value_of("pages").expect("pages").parse().unwrap();
                let bumplimit = matches
                    .value_of("bumplimit")
                    .expect("bumplimit")
                    .parse()
                    .unwrap();
                let per_page = matches
                    .value_of("per_page")
                    .expect("per_page")
                    .parse()
                    .unwrap();

                let (conn, _) = models::IBoard {
                    lang,
                    short,
                    last_replies,
                    name,
                    description,
                    rules,
                    pages_count,
                    bumplimit,
                    per_page,
                    thread_creating_limited: 0,
                    slowmode,
                    last_thread_created: time_now() - chrono::Duration::minutes(60),
                    op_oppost_enabled: matches.is_present("op-oppost"),
                    op_deletion_enabled: matches.is_present("op-deletion"),
                }
                .create(conn)
                .await?;
            }
            _ => panic!("Available commands: ['new']"),
        },

        "token" => match matches.value_of("command").unwrap() {
            "new" => {
                let roles = matches.value_of("roles").unwrap_or("").split(',');

                let (mut conn, token) = models::IToken::random(conn).await?;
                // FIXME: wherein
                for r in roles {
                    let (c, role) = models::Role::find_by_name(r, conn).await?;
                    let role = role.expect(&format!("Cannot find role `{}`", r));
                    conn = token.assign(&role, c).await?;
                }

                println!("{}", token.uuid);
            }

            "assign" => {
                let token_uuid = Uuid::parse_str(matches.value_of("token").unwrap()).unwrap();
                let role_name = matches.value_of("role").unwrap();
                let (conn, token) = models::Token::find_by_uuid(&token_uuid, conn).await?;
                let (conn, role) = models::Role::any(role_name, conn).await?;

                let _ = token.unwrap().assign(&role, conn).await?;
            }

            _ => panic!("Available commands: ['new']"),
        },

        "role" => match matches.value_of("command").unwrap() {
            "new" => {
                let name = matches.value_of("name").unwrap();
                let _ = models::Role::any(name, conn).await?;
            }

            "add-perm" => {
                let role_name = matches.value_of("role").unwrap();
                let perm_name = matches.value_of("perm").unwrap();

                let (conn, role) = models::Role::any(role_name, conn).await?;
                let (conn, perm) = models::Perm::any(perm_name, conn).await?;
                let conn = role.assign(&perm, conn).await?;
            }

            "remove-perm" => {
                let role_name = matches.value_of("role").unwrap();
                let perm_name = matches.value_of("perm").unwrap();

                let (conn, role) = models::Role::any(role_name, conn).await?;
                let (conn, perm) = models::Perm::any(perm_name, conn).await?;
                let conn = role.remove_perm(&perm, conn).await?;
            }

            _ => panic!("Available commands: ['add-perm']"),
        },

        "perm" => match matches.value_of("command").unwrap() {
            "new" => {
                let perm = matches.value_of("name").unwrap();
                let _ = models::Perm::any(perm, conn).await?;
            }

            _ => panic!("Available commands: ['new']"),
        },

        _ => panic!("Available sections: ['token']"),
    }

    Ok(())
}
