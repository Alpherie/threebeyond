extern crate threebeyond;

use {
    actix::Actor,
    actix_files::NamedFile,
    actix_web::{App, HttpServer},
    clap::{App as ClapApp, Arg as ClapArg},
    desumark::{Layout, Pattern, Segment},
    http::header::HeaderName,
    threebeyond::*,
    trust_dns_resolver::{config::*, TokioAsyncResolver},
};

#[actix_rt::main]
async fn main() -> Result<(), mysql_async::error::Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // clap
    let matches = ClapApp::new("threebeyond server daemon")
        .author("qch")
        .arg(
            ClapArg::with_name("target")
                .long("target")
                .short("t")
                .takes_value(true)
                .required(true),
        )
        .arg(
            ClapArg::with_name("db")
                .long("database_url")
                .short("d")
                .takes_value(true)
                .required(true),
        )
        .arg(
            ClapArg::with_name("cors")
                .long("cors_url")
                .short("a")
                .takes_value(true),
        )
        .get_matches();

    // get target
    let target = matches.value_of("target").unwrap().to_string();
    let db_url = matches.value_of("db").unwrap();
    let cors_target = matches.value_of("cors").unwrap_or(&target).to_string();

    /* build pool */
    let pool = Data::new(mysql_async::Pool::new(db_url));

    // get connection and create appstate
    let conn = pool.get_conn().await?;
    let app_state = Data::new(AppState::new(conn).await?.1);

    actors::DCaptchaClear {
        app_state: Data::clone(&app_state),
    }
    .start();

    actors::ThreadCountClear {
        app_state: Data::clone(&app_state),
    }
    .start();

    let watermark = Data::new(Watermark(
        image::open("assets/watermark.png").expect("Failed to load watermark"),
    ));

    // create http server
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())

            .app_data(Data::clone(&pool))
            .app_data(Data::clone(&app_state))
            .app_data(Data::clone(&watermark))

            .data(Layout::<config::MarkupErrorType>::new()
                .pattern(
                    "b",
                    Pattern::new()
                        .segment(Segment::Static("<b>"))
                        .segment(Segment::Inner)
                        .segment(Segment::Static("</b>")),
                )
                .unwrap()
                .pattern(
                    "highlight",
                    Pattern::new()
                        .segment(Segment::Static("<span class=\"highlight\">"))
                        .segment(Segment::Inner)
                        .segment(Segment::Static("</span>")),
                ).unwrap()
                .pattern(
                    "i",
                    Pattern::new()
                        .segment(Segment::Static("<i>"))
                        .segment(Segment::Inner)
                        .segment(Segment::Static("</i>")),
                )
                .unwrap()
                .pattern(
                    "urand",
                    Pattern::new()
                        .segment(Segment::Static("<span class=\"mark-urand\">"))
                        .segment(Segment::Inner)
                        .segment(Segment::Static("</span>")),
                )
                .unwrap()
                .pattern(
                    "textwall",
                    Pattern::new()
                        .segment(Segment::Static("<div class=\"textwall\"><div class=\"title\">"))
                        .segment(Segment::Linked(String::from("title")))
                        .segment(Segment::Static("</div><div class=\"content\">"))
                        .segment(Segment::Inner)
                        .segment(Segment::Static("</div></div>")),
                )
                .unwrap()
                .pattern(
                    "spoiler",
                    Pattern::new()
                        .segment(Segment::Static("<span class=\"spoiler\">"))
                        .segment(Segment::Inner)
                        .segment(Segment::Static("</span>")),
                )
                .unwrap()
                // .pattern(
                //     "textwall",
                //     Pattern::new()
                //         .segment(Segment::Static("<div class=\"textwall\"><b>"))
                //         .segment(Segment::Linked(String::from("title")))
                //         .segment(Segment::Static("</b>"))
                //         .segment(Segment::Inner)
                //         .segment(Segment::Static("</div>")),
                // ).unwrap()
            )

            .wrap(actix_cors::Cors::new()
                .allowed_origin(&cors_target)
                .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                .allowed_headers(vec![
                    actix_web::http::header::CONTENT_TYPE,
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::ACCEPT,
                    HeaderName::from_static("x-thread-secret"),
                    HeaderName::from_static("x-captcha-kind"),
                    HeaderName::from_static("x-captcha-value"),
                ])
                .expose_headers(vec![
                    HeaderName::from_static("x-dcaptcha-prefix"),
                    HeaderName::from_static("x-dcaptcha-hash"),
                    HeaderName::from_static("x-dcaptcha-lifetime"),
                ])
                .max_age(3600)
                .finish()
            )

            .data(markup::Parser::default())

            .data_factory(|| TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default()))

            .service(web::scope("res")
                .service(actix_files::Files::new("/uploads", "uploads/full"))
                .service(actix_files::Files::new("/thumbs", "uploads/thumbs"))
            )

            .service(web::scope("api")
                .wrap(middlewares::Authorization)
                .wrap(actix_web::middleware::DefaultHeaders::new().header("Cache-Control", "no-store"))
                .service(web::scope("captcha")
                    .service(routes::captcha::dcaptcha_generate)
                )
                // .service(routes::test_ban)

                /* get reasons */
                .service(routes::ban_reasons::list)

                /* system */
                .service(web::scope("system")
                    /* put settings */
                    .service(routes::system::put_settings)
                    
                    /* get settings */
                    .service(routes::system::get_settings)
                    )

                /* reasons */
                .service(web::scope("reasons")
                    /* {reason} */
                    .service(routes::ban_reasons::create)
                    )

                /* get ip */
                // .service(routes::ip_info)

                /* stats */
                .service(routes::stats_json)

                /* get auth */
                .service(routes::auth::index_json)

                .service(web::scope("roles")
                    /* post . */
                    .service(routes::auth::roles::create_json)

                    /* get {role} */
                    .service(routes::auth::roles::info)
                    .service(web::scope("{role}")

                        .service(web::scope("perms")
                            /* post . */
                            .service(routes::auth::roles::add_perm_json)
                            .service(web::scope("{perm}")
                                .service(routes::auth::roles::remove_perm)
                                )
                            )
                        )
                    )

                .service(web::scope("tokens")
                    /* post . */
                    .service(routes::auth::tokens::create_json)

                    /* dynamic */
                    .service(web::scope("{token}")
                        /* delete . */
                        .service(routes::auth::tokens::delete_json)

                        /* roles */
                        .service(web::scope("roles")
                            /* post . */
                            .service(routes::auth::tokens::add_role_json)

                            /* delete {role} */
                            .service(routes::auth::tokens::delete_role_json)
                            )
                        )
                    )

                .service(web::scope("counting")
                    /* get {[posts]} */
                    .service(routes::counting::get)
                )

                /* get langs*/
                .service(routes::langs::list_json)

                .service(web::scope("langs")                    
                    .service(web::scope("{lang}")
                        /* get boards */
                        .service(routes::langs::boards::list_json)

                        .service(web::scope("boards")
                            /* post . */
                            .service(routes::langs::boards::create_json)

                            /* update {board} */
                            .service(routes::langs::boards::update_json)

                            /* delete {board} */
                            .service(routes::langs::boards::delete_json)                            

                            /* get {board} */
                            .service(routes::langs::boards::get_json)
                            .service(web::scope("{board}")
                                /* threads */
                                .service(routes::langs::boards::threads::catalog_json)

                                /* reports */
                                .service(web::scope("reports")
                                    .service(web::scope("pages")
                                        /* get {page} */
                                        .service(routes::langs::boards::reports::pages::page)
                                        )

                                    .service(web::scope("{report}")
                                        /* post solve */
                                        .service(routes::langs::boards::reports::report::solve)
                                        )
                                    )

                                .service(web::scope("posts")
                                    /* get {} */
                                    .service(routes::langs::boards::posts::get_json)

                                    /* delete {} */
                                    .service(routes::langs::boards::posts::delete_json)

                                    /* delete [{posts}] */
                                    // .service(routes::langs::boards::posts::delete_multiple_json)

                                    .service(web::scope("[{posts}]")
                                        /* post report */
                                        .service(routes::langs::boards::posts::report_multiple_json)
                                        )

                                    .service(web::scope("{post}")
                                        /* post ban */
                                        .service(routes::langs::boards::posts::ban_json)
                                        /* get post/details */
                                        .service(routes::langs::boards::posts::details)
                                        )
                                    )

                                .service(web::scope("pages")
                                    /* get {page} */
                                    .service(routes::langs::boards::pages::get_json)
                                    )

                                .service(web::scope("threads")
                                    /* post . */
                                    .service(web::resource("")
                                        .wrap(middlewares::captcha::Captcha { reply: false })
                                        .wrap(middlewares::PostTimeoutCheck)
                                        .wrap(middlewares::CheckPostBan)
                                        .route(web::post().to(routes::langs::boards::threads::create_json))
                                        )

                                    /* get {num}*/
                                    .service(routes::langs::boards::threads::get_json)

                                    .service(web::scope("{num}")
                                        /* post close */
                                        .service(routes::langs::boards::threads::close)

                                        /* post open */
                                        .service(routes::langs::boards::threads::open)

                                        /* post pin */
                                        .service(routes::langs::boards::threads::pin)

                                        /* post unpin */
                                        .service(routes::langs::boards::threads::unpin)

                                        /* post endless */
                                        .service(routes::langs::boards::threads::endless)

                                        .service(web::scope("pages")
                                            /* get {} */
                                            .service(routes::langs::boards::threads::posts::get_page)
                                        )
                                        
                                        .service(web::scope("posts")
                                            /* delete {post}*/
                                            .service(routes::langs::boards::threads::posts::delete_json)
                                            
                                            /* post . */
                                            .service(web::resource("")
                                                .wrap(middlewares::captcha::Captcha { reply: true })
                                                .wrap(middlewares::PostTimeoutCheck)
                                                .wrap(middlewares::CheckPostBan)
                                                .route(web::post().to(routes::langs::boards::threads::posts::create_json))
                                                )

                                            .service(web::scope("{post}")
                                                /* post pin */
                                                .service(routes::langs::boards::threads::posts::pin)
                                            
                                                /* post unpin */
                                                .service(routes::langs::boards::threads::posts::unpin)
                                            )

                                            /* get count(optional: ?from_num={num}) */
                                            // .service(routes::langs::boards::threads::posts::count_json)
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            .service(actix_files::Files::new("/", "./dist/").index_file("index.html"))
            .default_service(web::route().to(|| async {
                NamedFile::open("dist/index.html")
            }))
    })
    .bind(target)?
    .run()
    .await?;

    Ok(())
}
