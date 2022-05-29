extern crate cmd_lib;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

extern crate actix;
extern crate actix_rt;
extern crate actix_web;
extern crate clap;
extern crate futures_util;
extern crate mysql_async;

#[macro_use]
extern crate validator_derive;
extern crate validator;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

pub mod actors;
pub mod api_error;
pub mod app_state;
pub mod config;
pub mod emailoptions;
pub mod extractors;
pub mod markup;
pub mod markup_functions;
pub mod middlewares;
pub mod models;
pub mod perm_rep;
pub mod response;
pub mod routes;
pub mod utils;

pub struct Watermark(pub image::DynamicImage);

pub use {
    crate::futures_util::{FutureExt, TryFutureExt},
    actix_multipart::Multipart,
    actix_web::{
        delete, get,
        http::StatusCode,
        post, put,
        web::{self, Data, Form, Json, Path, Query},
        Error, FromRequest, HttpRequest, HttpResponse, Responder,
    },
    api_error::ApiError,
    app_state::AppState,
    async_trait::async_trait,
    chrono::{Duration, NaiveDateTime},
    emailoptions::EmailOptionFn,
    image::DynamicImage,
    log::{info, trace, warn},
    mysql_async::{error::Error as MysqlError, prelude::*, Conn, FromRowError, Pool, Row},
    rand::Rng,
    regex::Regex,
    serde::Deserialize,
    serde_json::Value,
    std::pin::Pin,
    std::{collections::HashMap, future::Future, sync::Arc},
    tokio::{fs, sync::Mutex},
    uuid::Uuid,
    validator::{Validate, ValidationError},
};

pub type TokioDnsResolver = trust_dns_resolver::AsyncResolver<
    trust_dns_resolver::TokioConnection,
    trust_dns_resolver::TokioConnectionProvider,
>;

lazy_static! {
    static ref REGEX_EMAIL: Regex = Regex::new("^^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$$").unwrap();
}

pub fn merge_json(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge_json(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b;
}

pub fn time_now() -> chrono::NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
