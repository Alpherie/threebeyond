use beyond;

/* langs */
DROP TABLE IF EXISTS `langs`;
CREATE TABLE `langs` (
    code          VARCHAR(8) PRIMARY KEY,
    name          VARCHAR(32) NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);

/* donations */
DROP TABLE IF EXISTS `donations`;
CREATE TABLE `donations` (
    id            BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name          VARCHAR(64) NOT NULL,
    currency      VARCHAR(64) NOT NULL,
    amount        BIGINT UNSIGNED NOT NULL,
    amount_usd    BIGINT UNSIGNED NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);

/* ips */
DROP TABLE IF EXISTS `ips`;
CREATE TABLE `ips` (
    id            BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    addr          VARBINARY(16) NOT NULL,
    last          TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP),
    first         TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);
CREATE UNIQUE INDEX IF NOT EXISTS `addr` on `ips`(`addr`);

/* ban_reasons */
DROP TABLE IF EXISTS `ban_reasons`;
CREATE TABLE `ban_reasons` (
    id            BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    alias         VARCHAR(64) NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);
CREATE UNIQUE INDEX IF NOT EXISTS `alias` on `ban_reasons`(`alias`);

/* ip6_bans */
DROP TABLE IF EXISTS `ip6_bans`;
CREATE TABLE `ip6_bans` (
    uuid          VARBINARY(16) PRIMARY KEY,
    subnet        BIGINT UNSIGNED NOT NULL,             -- 48 NETWORK ID + 16 SUBNET ID
    device        BIGINT UNSIGNED,                      -- IF NULL THEN WHOLE SUBNET IS BANNED
    lang          VARCHAR(8),                           -- IF NULL THEN BANNED ON WHOLE SERVICE
    board         BIGINT UNSIGNED,                      -- IF NULL THEN BANNED ON WHOLE LANG DOMAIN
    because       BIGINT UNSIGNED,                      -- DIRECT ID TO POST BECAUSE USER HAS BEEN BANNED
    reason        BIGINT UNSIGNED NOT NULL,             -- DIRECT ID TO REASON
    comment       VARCHAR(128),                         -- COMMENT WHY USER HAS BEEN BANNED
    lasts         TIMESTAMP NOT NULL,                   -- DATETIME WHEN USER WILL BE UNBANNED
    created_at    TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);
CREATE INDEX IF NOT EXISTS `subnet` on `ip6_bans`(`subnet`);
CREATE INDEX IF NOT EXISTS `lang` on `ip6_bans`(`lang`);
CREATE INDEX IF NOT EXISTS `board` on `ip6_bans`(`board`);
CREATE INDEX IF NOT EXISTS `lang_board` on `ip6_bans`(`lang`, `board`);
CREATE INDEX IF NOT EXISTS `reason` on `ip6_bans`(`reason`);

/* ip4_bans */
DROP TABLE IF EXISTS `ip4_bans`;
CREATE TABLE `ip4_bans` (
    uuid          VARBINARY(16) PRIMARY KEY,
    subnet        SMALLINT UNSIGNED NOT NULL,
    device        SMALLINT UNSIGNED,                    -- IF NULL THEN WHOLE SUBNET IS BANNED
    lang          VARCHAR(8),                           -- IF NULL THEN BANNED ON WHOLE SERVICE
    board         BIGINT UNSIGNED,                      -- IF NULL THEN BANNED ON WHOLE LANG DOMAIN
    because       BIGINT UNSIGNED,                      -- DIRECT ID TO POST BECAUSE USER HAS BEEN BANNED
    reason        BIGINT UNSIGNED NOT NULL,             -- DIRECT ID TO REASON
    comment       VARCHAR(128),                         -- COMMENT WHY USER HAS BEEN BANNED
    lasts         TIMESTAMP NOT NULL,                   -- DATETIME WHEN USER WILL BE UNBANNED
    created_at    TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);
CREATE INDEX IF NOT EXISTS `subnet` on `ip4_bans`(`subnet`);
CREATE INDEX IF NOT EXISTS `lang` on `ip4_bans`(`lang`);
CREATE INDEX IF NOT EXISTS `board` on `ip4_bans`(`board`);
CREATE INDEX IF NOT EXISTS `lang_board` on `ip4_bans`(`lang`, `board`);
CREATE INDEX IF NOT EXISTS `reason` on `ip4_bans`(`reason`);

/* token bans */
DROP TABLE IF EXISTS `token_bans`;
CREATE TABLE `token_bans` (
    uuid          VARBINARY(16) PRIMARY KEY,
    token         BIGINT UNSIGNED NOT NULL,
    lang          VARCHAR(8),                           -- IF NULL THEN BANNED ON WHOLE SERVICE
    board         BIGINT UNSIGNED,                      -- IF NULL THEN BANNED ON WHOLE LANG DOMAIN
    because       BIGINT UNSIGNED,                      -- DIRECT ID TO POST BECAUSE USER HAS BEEN BANNED
    reason        BIGINT UNSIGNED NOT NULL,             -- DIRECT ID TO REASON
    comment       VARCHAR(128),                         -- COMMENT WHY USER HAS BEEN BANNED
    lasts         TIMESTAMP NOT NULL,                   -- DATETIME WHEN USER WILL BE UNBANNED
    created_at    TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);
CREATE INDEX IF NOT EXISTS `token` on `token_bans`(`token`);
CREATE INDEX IF NOT EXISTS `lang` on `token_bans`(`lang`);
CREATE INDEX IF NOT EXISTS `board` on `token_bans`(`board`);
CREATE INDEX IF NOT EXISTS `lang_board` on `token_bans`(`lang`, `board`);
CREATE INDEX IF NOT EXISTS `reason` on `token_bans`(`reason`);


/* boards */
DROP TABLE IF EXISTS `boards`;
CREATE TABLE `boards` (
    id            BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    lang          VARCHAR(8) NOT NULL,
    short         VARCHAR(16) NOT NULL,
    name          TEXT NOT NULL,
    description   TEXT NOT NULL,
    rules         TEXT NOT NULL,
    pages_count   BIGINT UNSIGNED NOT NULL DEFAULT(10),
    per_page      BIGINT UNSIGNED NOT NULL DEFAULT(10),
    last_replies  BIGINT UNSIGNED NOT NULL DEFAULT(3),
    count         BIGINT UNSIGNED NOT NULL DEFAULT(0),
    bumplimit     INTEGER UNSIGNED NOT NULL DEFAULT(200),
    thread_creating_limited SMALLINT UNSIGNED NOT NULL DEFAULT(0),
    slowmode      INTEGER UNSIGNED,
    last_thread_created TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP),
    op_oppost_enabled   BOOLEAN NOT NULL DEFAULT(0),
    op_deletion_enabled BOOLEAN NOT NULL DEFAULT(0),
    tripcode_enabled BOOLEAN NOT NULL DEFAULT(0),
    INDEX lang_short (lang, short)
);

CREATE INDEX IF NOT EXISTS `_short` ON boards(short);

/* threads */
DROP TABLE IF EXISTS `threads`;
CREATE TABLE `threads` (
    id          BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    owner       BIGINT UNSIGNED,
    ip          BIGINT UNSIGNED NOT NULL,
    board_id    BIGINT UNSIGNED NOT NULL,
    post_id     BIGINT UNSIGNED NOT NULL,
    post_num    BIGINT UNSIGNED NOT NULL,
    sticky      INTEGER UNSIGNED NOT NULL,          -- NOT bool because boolean doesNOT support index
    endless     BOOLEAN NOT NULL DEFAULT(0),
    closed      SMALLINT UNSIGNED NOT NULL DEFAULT(0),
    secret      BIGINT UNSIGNED NOT NULL,
    count       BIGINT UNSIGNED NOT NULL DEFAULT(1),
    lasthit     TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);

CREATE INDEX IF NOT EXISTS `_closed` ON threads(closed);
CREATE INDEX IF NOT EXISTS `_sticky` ON threads(sticky);
CREATE INDEX IF NOT EXISTS `_post_id` ON threads(post_id);
CREATE INDEX IF NOT EXISTS `_post_num` ON threads(post_num);

/* posts */
DROP TABLE IF EXISTS `posts`;
CREATE TABLE `posts` (
    id          BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    owner       BIGINT UNSIGNED,
    ip          BIGINT UNSIGNED NOT NULL,
    board_id    BIGINT UNSIGNED NOT NULL,
    parent_id   BIGINT UNSIGNED,
    parent_num  BIGINT UNSIGNED,
    num         BIGINT UNSIGNED NOT NULL,
    pinned      BOOLEAN NOT NULL DEFAULT(0),
    sage        BOOLEAN NOT NULL DEFAULT(0),
    op          BOOLEAN NOT NULL DEFAULT(0),
    moder       BOOLEAN NOT NULL DEFAULT(0),
    trip        TEXT,
    subject     TEXT,
    name        TEXT,
    email       TEXT,
    message     TEXT,
    good        BIGINT UNSIGNED NOT NULL DEFAULT(0),
    bad         BIGINT UNSIGNED NOT NULL DEFAULT(0),
    at          TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);

CREATE INDEX IF NOT EXISTS `_parent_id` ON posts(parent_id);
CREATE INDEX IF NOT EXISTS `_parent_num` ON posts(parent_num);
CREATE INDEX IF NOT EXISTS `_num` ON posts(num);

/* AUTHORIZATION SYSTEM */
-- tokens
DROP TABLE IF EXISTS `tokens`;
CREATE TABLE tokens (
    id          BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    is_mod      SMALLINT UNSIGNED NOT NULL DEFAULT(0),
    uuid        VARBINARY(16) UNIQUE NOT NULL,
    at          TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);
CREATE INDEX IF NOT EXISTS `is_mod` ON tokens(is_mod);
CREATE INDEX IF NOT EXISTS `uuid` ON tokens(uuid);

-- roles
DROP TABLE IF EXISTS `roles`;
CREATE TABLE roles (
    id          BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name        VARCHAR(64) UNIQUE NOT NULL
);
CREATE INDEX IF NOT EXISTS `name` ON roles(name);

-- permissions
DROP TABLE IF EXISTS `perms`;
CREATE TABLE perms (
    id          BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name        VARCHAR(64) NOT NULL
);
CREATE INDEX IF NOT EXISTS `name` ON perms(name);

-- relationships
DROP TABLE IF EXISTS `role_perms`;
CREATE TABLE role_perms(
    role        BIGINT UNSIGNED NOT NULL,
    perm        BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (role,perm)
);

DROP TABLE IF EXISTS `token_roles`;
CREATE TABLE token_roles(
    token       BIGINT UNSIGNED NOT NULL,
    role        BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (token,role)
);

DROP TABLE IF EXISTS `files`;
CREATE TABLE files(
    uuid        VARBINARY(16) PRIMARY KEY,
    post        INTEGER UNSIGNED NOT NULL,
    name        VARCHAR(256) NOT NULL,
    extension   VARCHAR(64) NOT NULL,
    width       INTEGER UNSIGNED NOT NULL,
    height      INTEGER UNSIGNED NOT NULL,
    size        BIGINT UNSIGNED NOT NULL
);

CREATE INDEX IF NOT EXISTS `_post` ON files(post);

DROP TABLE IF EXISTS `reports`;
CREATE TABLE reports(
    id          BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    ignored     SMALLINT UNSIGNED NOT NULL,
    solved_at   TIMESTAMP,
    solved_by   BIGINT UNSIGNED,
    owner       BIGINT UNSIGNED,
    ip          BIGINT UNSIGNED NOT NULL,
    lang        VARCHAR(8) NOT NULL,
    board_id    BIGINT UNSIGNED NOT NULL,
    board_short VARCHAR(16) NOT NULL,
    thread_id   BIGINT UNSIGNED NOT NULL,
    thread_num  BIGINT UNSIGNED NOT NULL,
    comment     VARCHAR(64) NOT NULL,
    at          TIMESTAMP NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);

CREATE INDEX IF NOT EXISTS `_ignored` ON reports(ignored);
CREATE INDEX IF NOT EXISTS `_lang` ON reports(lang);
CREATE INDEX IF NOT EXISTS `_board_id` ON reports(board_id);
CREATE INDEX IF NOT EXISTS `_solved_by` ON reports(solved_by);

DROP TABLE IF EXISTS `report_post_nums`;
CREATE TABLE report_post_nums(
    id          BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    report_id   BIGINT UNSIGNED NOT NULL,
    post_num    BIGINT UNSIGNED NOT NULL
);

CREATE INDEX IF NOT EXISTS `_report_id` ON report_post_nums(report_id);
CREATE INDEX IF NOT EXISTS `_post_num` ON report_post_nums(post_num);
