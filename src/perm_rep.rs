pub const general_board_perms: &[&str] = &[
    "posts::delete",
    "posts::ban",
    "posts::ban_subnet",
    "posts::view_details",
    "posting::create_thread",
    "threads::open",
    "threads::close",
    "threads::pin",
    "threads::unpin",
    "threads::reply",
    "threads::reply_closed",
    "threads::make_endless",
    "reports::list",
    "reports::solve",
];

pub const general_lang_perms: &[&str] = &["boards::create"];

pub const admin_board_perms: &[&str] = &["control"];
