/* INTERVALS (in ms)*/
pub static BOARD_CACHE_UPDATE_INTERVAL: i64 = 3000;
pub static BOARD_CACHE_COLLECTOR_INTERVAL: i64 = 5000;
pub static APPSTATE_STATS_CACHE_INTERVAL: u64 = 5000;
// pub static UNUSED_ATTACHMENTS_COLLECTOR_INTERVAL: i64   = 30000;
// pub static POST_DELAY                           : i64   = 10000;

/* REPORT SYSTEM */
pub static MAX_REPORTS: u64 = 2;
pub static REPORTS_PER_PAGE: u64 = 10;

pub static HCAPTCHA_URL: &str = "https://hcaptcha.com/siteverify";
pub static HCAPTCHA_SITE_KEY: &str = "39636c74-4555-4686-b879-40912f147ac3";
pub static HCAPTCHA_SECRET_KEY: &str = "0x975718189b040D5f19e39B1Fd503584dBA18Cebb";

/* CAPTCHA */
pub static DIGIT_CAPTCHA_LIFETIME: i64 = 30; // secs
pub static DIGIT_CAPTCHA_CLEAR_INTERVAL: u64 = 5; // secs
pub static DCAPTCHA_PREFIX_LENGTH: usize = 16;
pub static DCAPTCHA_PREFIX_CHARSET: &[u8] = b"ABCDEF";

/* THREAD POSTS COUNT CACHE */
pub static THREAD_COUNT_CACHE_LIFETIME: i64 = 60; // secs
pub static THREAD_COUNT_CACHE_CLEAR_INTERVAL: u64 = 30; // secs
pub static THREAD_COUNT_MAX_PER_REQUEST: usize = 100;

pub static TRIPCODE_SECRET: &str = "e934c8e2eaf5e71aa30bc87fdabedecb46a4914a70be2f503be332dde1e021650cd12c79";

/* MULTIPART */
pub static THUMBNAIL_SIZE: u32 = 128;
pub static THUMBNAIL_EXTENSION: &str = "png";
pub static MAX_FILES_PER_POST: usize = 4;
pub static MAX_FILE_SIZE: usize = (10 * 1024 * 1024);
pub static MAX_PAYLOAD_SIZE: usize = (10 * 1024 * 1024);
pub static WATERMARK_ENABLED: bool = true;
pub static WATERMARK_RATIO: u32 = 5;
pub static WATERMARK_OFFSET_RATIO: u32 = 2;
pub static WATERMARK_RANDOMIZE_COLOR: bool = true;
pub type MarkupErrorType = u8;

pub static POST_TIMEOUT: u128 = 30000;
