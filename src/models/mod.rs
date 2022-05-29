use super::*;

use std::net::IpAddr;

use threebeyond_codegen_from_row::FromRow;

macro_rules! import {
    ([$($module:ident),*]) => {
        $(
            pub mod $module;
            pub use $module::*;
        )*
    };
}

import!([
    report, board, donation, file, ip, lang, perm, post, role, thread, token, ban_reason, ban,
    token_ban
]);

#[async_trait]
pub trait InsertMultiple<T>
where
    T: Sized,
{
    async fn commit(self, conn: Conn) -> Result<Conn, MysqlError>;
}

use serde::ser::{Serialize, SerializeStruct, Serializer};

fn row_to_thread_post_as_reply(row: mysql_async::Row) -> Thread {
    let mut thread = Thread {
        id: row.get("thread_id").unwrap(),
        owner: row.get("owner").unwrap(),
        ip: row.get("ip").unwrap(),
        board_id: row.get("board_id").unwrap(),
        post_id: row.get("post_id").unwrap(),
        post_num: row.get("post_num").unwrap(),
        closed: row.get("thread_closed").unwrap(),
        endless: row.get("thread_endless").unwrap(),
        sticky: row.get("thread_sticky").unwrap(),
        secret: row.get("thread_secret").unwrap(),
        count: row.get("count").unwrap(),
        posts: None,
        lasthit: row.get("thread_lasthit").unwrap(),
    };

    let post = Post {
        id: row.get("post_id").unwrap(),
        owner: row.get("owner").unwrap(),
        ip: row.get("ip").unwrap(),
        board_id: row.get("board_id").unwrap(),
        parent_id: None,
        parent_num: None,
        num: row.get("post_num").unwrap(),
        pinned: row.get("post_pinned").unwrap(),
        sage: row.get("post_sage").unwrap(),
        op: row.get("post_op").unwrap(),
        moder: row.get("post_moder").unwrap(),
        trip: row.get("post_trip").unwrap(),
        subject: row.get("post_subject").unwrap(),
        name: row.get("post_name").unwrap(),
        email: row.get("post_email").unwrap(),
        message: row.get("post_message").unwrap(),
        good: row.get("post_good").unwrap(),
        bad: row.get("post_bad").unwrap(),
        at: row.get("at").unwrap(),

        files: None,
    };

    thread.posts = Some(vec![post]);

    thread
}

fn row_to_thread_post_as_reply_to_value(row: mysql_async::Row) -> Value {
    let thread = Thread {
        id: row.get("thread_id").unwrap(),
        owner: row.get("owner").unwrap(),
        ip: row.get("ip").unwrap(),
        board_id: row.get("board_id").unwrap(),
        post_id: row.get("post_id").unwrap(),
        post_num: row.get("post_num").unwrap(),
        closed: row.get("thread_closed").unwrap(),
        sticky: row.get("thread_sticky").unwrap(),
        endless: row.get("thread_endless").unwrap(),
        secret: row.get("thread_secret").unwrap(),
        count: row.get("count").unwrap(),
        posts: None,
        lasthit: row.get("thread_lasthit").unwrap(),
    };

    let post = Post {
        id: row.get("post_id").unwrap(),
        owner: row.get("owner").unwrap(),
        ip: row.get("ip").unwrap(),
        board_id: row.get("board_id").unwrap(),
        parent_id: None,
        parent_num: None,
        num: row.get("post_num").unwrap(),
        pinned: row.get("post_pinned").unwrap(),
        sage: row.get("post_sage").unwrap(),
        op: row.get("post_op").unwrap(),
        moder: row.get("post_moder").unwrap(),
        trip: row.get("post_trip").unwrap(),
        subject: row.get("post_subject").unwrap(),
        name: row.get("post_name").unwrap(),
        email: row.get("post_email").unwrap(),
        message: row.get("post_message").unwrap(),
        good: row.get("post_good").unwrap(),
        bad: row.get("post_bad").unwrap(),
        at: row.get("at").unwrap(),

        files: None,
    };

    let mut a = json!(thread);
    crate::merge_json(&mut a, json!({ "posts": [post] }));

    a
}

fn row_to_thread_post(row: mysql_async::Row) -> (Thread, Post) {
    let thread = Thread {
        id: row.get("thread_id").unwrap(),
        owner: row.get("owner").unwrap(),
        ip: row.get("ip").unwrap(),
        board_id: row.get("board_id").unwrap(),
        post_id: row.get("post_id").unwrap(),
        post_num: row.get("post_num").unwrap(),
        closed: row.get("thread_closed").unwrap(),
        endless: row.get("thread_endless").unwrap(),
        sticky: row.get("thread_sticky").unwrap(),
        secret: row.get("thread_secret").unwrap(),
        count: row.get("count").unwrap(),
        posts: None,
        lasthit: row.get("thread_lasthit").unwrap(),
    };

    let post = Post {
        id: row.get("post_id").unwrap(),
        owner: row.get("owner").unwrap(),
        ip: row.get("ip").unwrap(),
        board_id: row.get("board_id").unwrap(),
        parent_id: None,
        parent_num: None,
        num: row.get("post_num").unwrap(),
        pinned: row.get("post_pinned").unwrap(),
        sage: row.get("post_sage").unwrap(),
        op: row.get("post_op").unwrap(),
        moder: row.get("post_moder").unwrap(),
        trip: row.get("post_trip").unwrap(),
        subject: row.get("post_subject").unwrap(),
        name: row.get("post_name").unwrap(),
        email: row.get("post_email").unwrap(),
        message: row.get("post_message").unwrap(),
        good: row.get("post_good").unwrap(),
        bad: row.get("post_bad").unwrap(),
        at: row.get("at").unwrap(),

        files: None,
    };

    (thread, post)
}
