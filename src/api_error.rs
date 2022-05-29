use super::*;
use actix_multipart::MultipartError;
use desumark::Error as MarkError;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt::Display;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    stage: Option<String>,
    code: String,
    message: String,
    details: Option<Value>,

    dump: Option<Box<dyn std::error::Error>>,
}

impl ApiError {
    pub fn new<S: ToString, C: ToString, M: ToString>(
        status: StatusCode,
        stage: Option<S>,
        code: C,
        message: M,
        details: Option<Value>,
    ) -> Self {
        Self {
            status: status,
            stage: match stage {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            code: code.to_string(),
            message: message.to_string(),
            details: details,
            dump: None,
        }
    }

    pub fn dump<E>(mut self, error: E) -> Self
    where
        E: std::error::Error + 'static,
    {
        self.dump = Some(Box::new(error));

        self
    }

    pub fn board_slowmode_remaining(seconds: i64) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            Some("THREAD_CREATE"),
            "BORAD_SLOWMODE_REMAINING",
            "You cannot create thread right now, try later.",
            Some(json!({ "left": seconds })),
        )
    }

    pub fn token_secret_invalid() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("THREAD_AUTH"),
            "THREAD_SECRET_INVALID",
            "Thread secret is not valid. Access denied.",
            None,
        )
    }

    pub fn last_post_timeout(millis: u128) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("POST_AVAILABLE"),
            "LAST_POST_TIMEOUT",
            &[
                "You have to wait ",
                &(millis / 1000).to_string(),
                " seconds before creating new post",
            ]
            .concat(),
            Some(json!({ "left": (millis / 1000) as u32 })),
        )
    }

    pub fn thread_secret_not_present() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("THREAD_AUTH"),
            "THREAD_SECRET_NOT_PRESENT",
            "Thread secret is not present. Access denied.",
            None,
        )
    }

    pub fn invalid_format() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("PARSE"),
            "INVALID_REQUEST_FORMAT",
            "<.<",
            None,
        )
    }

    /* constructors */
    pub fn ban(info: models::Ban, reason: models::BanReason) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("BAN_CHECK"),
            "BANNED",
            "You are banned and can't post here.",
            Some(json!({
                "lang": info.lang,
                "board": info.board,
                "reason": reason.alias,
                "comment": info.comment,
                "until": info.lasts.timestamp(),
                "at": info.created_at.timestamp()
            })),
        )
    }

    pub fn post_invalid_thread() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("POST_DELETE"),
            "POST_INVALID_THREAD",
            "Post is located outside the thread",
            None,
        )
    }

    pub fn token_ban(info: models::TokenBan, reason: models::BanReason) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("BAN_CHECK"),
            "BANNED_TOKEN",
            "You are banned and can't post here.",
            Some(json!({
                "lang": info.lang,
                "board": info.board,
                "reason": reason.alias,
                "comment": info.comment,
                "until": info.lasts.timestamp(),
                "at": info.created_at.timestamp()
            })),
        )
    }

    pub fn action_not_allowed_by_board_settings() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Option::<u8>::None,
            "ACTION_NOT_ALLOWED_BY_BOARD_SETTINGS",
            "This action is not allowed because of board settings",
            None,
        )
    }

    pub fn op_secret_invalid() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("OP_AUTHENTICATION"),
            "OP_SECRET_INVALID",
            "OP secret key is invalid",
            None,
        )
    }

    pub fn post_not_related_to_thread() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("THREAD_MODIFICATION"),
            "POST_NOT_RELATED_TO_THREAD",
            "The post is not related to the thread",
            None,
        )
    }

    pub fn report_select_not_same_thread() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("SELECT"),
            "REPORT_SELECT_NOT_SAME_THREAD",
            "Thread must be same for all posts.",
            None,
        )
    }

    pub fn captcha_kind_required() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("CAPTCHA"),
            "CAPTCHA_KIND_REQUIRED",
            "Kind of hcaptcha is required.",
            None,
        )
    }

    pub fn captcha_kind_unsupported() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("CAPTCHA"),
            "CAPTCHA_KIND_UNSUPPORTED",
            "Kind of hcaptcha is not supported.",
            None,
        )
    }

    pub fn captcha_value_required() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("CAPTCHA"),
            "CAPTCHA_VALUE_REQUIRED",
            "Captcha is required.",
            None,
        )
    }

    pub fn captcha_value_invalid() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("CAPTCHA"),
            "CAPTCHA_VALUE_INVALID",
            "Captcha is invalid.",
            None,
        )
    }

    pub fn thread_closed() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("POST"),
            "THREAD_CLOSED",
            "Thread is closed.",
            None,
        )
    }

    pub fn max_reports_amount_sent() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("REPORT"),
            "MAX_REPORTS_SENT",
            "You have already sent max amount of reports.",
            Some(json!({ "max": config::MAX_REPORTS })),
        )
    }

    pub fn multipart_file_corrupted(name: &str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MULTIPART"),
            "MULTIPART_FILE_CORRUPTED",
            format!("File from field {:?} is corrupted.", name),
            Some(json!({ "field": name })),
        )
    }

    pub fn banned_spamhaus() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("BLACKLIST_CHECK"),
            "IP_BLACKLISTED",
            "Your ip is blacklisted",
            None,
        )
    }

    pub fn multipart_unknown_extenstions(field: &str, ext: &str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MULTIPART"),
            "MULTIPART_UNKNOWN_EXTENSION",
            format!(
                "File from field {:?} has unknown or unsupported format: {:?}",
                field, ext
            ),
            Some(json!({ "field": field, "extension": ext })),
        )
    }

    pub fn multipart_file_bad_field_name(name: &str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MULTIPART"),
            "MULTIPART_FILE_BAD_FIELD_NAME",
            "File's Field's name must begin with `file`",
            Some(json!({ "field": name })),
        )
    }

    pub fn multipart_files_limit() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MULTIPART"),
            "MULTIPART_FILES_LIMIT",
            "You have reached limit of files",
            None,
        )
    }

    pub fn multipart_file_extension_too_long(field: &str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MULTIPART"),
            "MULTIPART_FILE_EXTENSION_TOO_LONG",
            format!(
                "Extension of file in field {:?} is longer than 64 bytes",
                field
            ),
            Some(json!({ "field": field })),
        )
    }

    pub fn multipart_filename_too_long(field: &str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MULTIPART"),
            "MULTIPART_FILENAME_TOO_LONG",
            format!(
                "Filename of file in field {:?} is longer than 256 bytes",
                field
            ),
            Some(json!({ "field": field })),
        )
    }

    pub fn multipart_field_overflow() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MULTIPART"),
            "MULTIPART_FIELD_OVERFLOW",
            "One of field is very big",
            None,
        )
    }

    pub fn report_not_found() -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND"),
            "REPORT_NOT_FOUND",
            "Cannot find report",
            None,
        )
    }

    pub fn report_already_solved() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("REPORT_CONTROL"),
            "REPORT_ALREADY_SOLVED",
            "Report is already solved",
            None,
        )
    }

    pub fn role_not_found<S: fmt::Debug + Serialize>(role: S) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND"),
            "ROLE_NOT_FOUND",
            format!("Cannot find role {:?}.", role),
            Some(json!({ "name": role })),
        )
    }

    pub fn cannot_create_role_duplicate(role: &str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("CREATE"),
            "CREATE_ROLE_FAILED_DUPLICATE",
            format!(
                "Cannot create role with name {:?} because role with this name already exists.",
                role
            ),
            Some(json!({ "name": role })),
        )
    }

    pub fn access_missing_perm(perm: &str) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("ACCESS"),
            "ACCESS_MISSING_PERM",
            format!("You don't have permission {:?} to do this", perm),
            Some(json!({ "perm": perm })),
        )
    }

    pub fn access_missing_role(role: &str) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("ACCESS"),
            "ACCESS_MISSING_ROLE",
            format!("You don't have role {:?} to do this", role),
            Some(json!({ "role": role })),
        )
    }

    pub fn authorization_bad_token() -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            Some("AUTH"),
            "AUTHORIZATION_BAD_TOKEN",
            "Authorization header is incorrect",
            None,
        )
    }

    pub fn authorization_required() -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            Some("AUTH"),
            "UNAUTHORIZED",
            "Authorization header is required",
            None,
        )
    }

    pub fn thread_is_readonly() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("THREAD_ACCESS_CHECK"),
            "THREAD_IS_READONLY",
            "Failed to reply thread because it is marked as readonly",
            None,
        )
    }

    pub fn validation_error(err: validator::ValidationErrors) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("VALIDATE_FIELDS"),
            "VALIDATION_ERROR",
            "Failed to validate request",
            Some(json!(err)),
        )
    }

    pub fn recaptcha_invalid() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("VALIDATE_CAPTCHA"),
            "RECAPTCHA_IS_INVALID",
            "Recaptcha response is bad",
            None,
        )
    }

    pub fn token_not_found(uuid: &Uuid) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_TOKEN"),
            "TOKEN_NOT_FOUND_SECRET",
            format!("Failed to find token by uuid {:?}", uuid),
            Some(json!({"uuid": uuid.to_hyphenated().to_string()})),
        )
    }

    pub fn page_not_found<S: ToString + std::fmt::Debug>(short: S, page: u64) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_PAGE"),
            "PAGE_NOT_FOUND",
            format!(
                "Failed to find page on board {:?} with index {:?}",
                short, page
            ),
            None,
        )
    }

    pub fn post_not_found(lang: impl Display, short: impl Display, thread: u64, num: u64) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_POST_IN_THREAD_ON_BOARD"),
            "POST_NOT_FOUND",
            format!(
                "Failed to find post on board `{}/{}` with num `{}` in thread `{}`",
                lang, short, num, thread
            ),
            None,
        )
    }

    pub fn thread_not_found<S: ToString + std::fmt::Display>(short: S, num: u64) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_THREAD"),
            "THREAD_NOT_FOUND",
            format!(
                "Failed to find thread on board `{}` with num `{}`",
                short, num
            ),
            None,
        )
    }

    pub fn ban_reason_not_found<R: AsRef<str>>(r: R) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_BAN_REASON"),
            "BAN_REASON_NOT_FOUND",
            ["Failed to find ban reason `", r.as_ref(), "`"].concat(),
            None,
        )
    }

    pub fn lang_not_found<L: AsRef<str>>(lang: L) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_LANG"),
            "LANG_NOT_FOUND",
            ["Failed to find language `", lang.as_ref()].concat(),
            None,
        )
    }

    pub fn board_already_exists<S: AsRef<str>, L: AsRef<str>>(short: S, lang: L) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("FIND_BOARD_DUPLICATE"),
            "BOARD_ALREADY_EXISTS",
            [
                "Board `",
                short.as_ref(),
                "` for language `",
                lang.as_ref(),
                "` already exists",
            ]
            .concat(),
            None,
        )
    }

    pub fn board_not_found<S: AsRef<str>, L: AsRef<str>>(short: S, lang: L) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_BOARD"),
            "BOARD_NOT_FOUND",
            [
                "Failed to find board `",
                short.as_ref(),
                "` for language `",
                lang.as_ref(),
            ]
            .concat(),
            None,
        )
    }

    pub fn post_not_found_on_board(short: &str, num: u64) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("FIND_POST"),
            "POST_NOT_FOUND",
            format!("We could not found post {:?} on board {:?}", num, short),
            Some(json!({"board": short, "num": num})),
        )
    }

    pub fn perm_not_found_name(perm: &str) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_PERM"),
            "PERM_NOT_FOUND_NAME",
            format!("Failed to find perm by name {:?}", perm),
            Some(json!({ "name": perm })),
        )
    }

    pub fn role_not_found_name(role: &str) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            Some("FIND_ROLE"),
            "ROLE_NOT_FOUND_NAME",
            format!("Failed to find role by name {:?}", role),
            Some(json!({ "name": role })),
        )
    }

    pub fn thread_create_perm_required() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("CHECK_PERMISSIONS"),
            "THREAD_CREATE_PERM_REQUIRED",
            "You can't create threads on this board",
            None,
        )
    }

    pub fn comment_or_attachment_required() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("POSTING"),
            "COMMENT_OR_ATTACHMENT_REQUIRED",
            "You have to add comment or attachment",
            None,
        )
    }

    pub fn attachment_required() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Some("POSTING"),
            "ATTACHMENT_REQUIRED",
            "You have to add attachment to your post",
            None,
        )
    }

    // pub fn banned_on_board(short: &str, err: &Ban, reason: &BanReason) -> Self {
    //     Self::new(StatusCode::FORBIDDEN, Some("BAN_CHECKING"), "BANNED_ON_BOARD", format!("#{} You are banned on board {:?} until {:?}, reason: {:?}.", err.id, short, err.expires.format("%Y-%m-%d %H:%M:%S").to_string(), reason.message), Some(json!({
    //         "ban":    err,
    //         "reason": reason
    //     })))
    // }
}

// impl From<ApiError> for ApiError {
//     fn from(t: ApiError) -> Self {
//         t
//     }
// }

impl From<validator::ValidationErrors> for ApiError {
    fn from(t: validator::ValidationErrors) -> Self {
        Self::validation_error(t)
    }
}

impl From<actix_web::Error> for ApiError {
    fn from(e: actix_web::Error) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("ACTIX"),
            "ACTIX_INTERNAL",
            "<hidden>",
            None,
        )
        .dump(e)
    }
}

impl From<MultipartError> for ApiError {
    fn from(e: MultipartError) -> Self {
        use MultipartError::*;

        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MULTIPART"),
            format!(
                "{}",
                match e {
                    NoContentType => "NO_CONTENT_TYPE",
                    ParseContentType => "PARSE_CONTENT_TYPE",
                    Boundary => "BOUNDARY",
                    Nested => "NESTED",
                    Incomplete => "INCOMPLETE",
                    _ => "OTHER",
                }
            ),
            format!("{}", e),
            None,
        )
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("JSON"),
            "JSON_ERROR",
            format!("{}", e),
            None,
        )
    }
}

impl From<http::header::ToStrError> for ApiError {
    fn from(_e: http::header::ToStrError) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("HEADER"),
            "INCORRECT",
            "Cannot parse header",
            None,
        )
    }
}

impl From<MysqlError> for ApiError {
    fn from(t: MysqlError) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("INTERNAL"),
            "SQL_ERROR",
            "Something very bad happened. You should notify the administartor",
            None,
        )
        .dump(t)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(_t: std::io::Error) -> ApiError {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("IO"),
            "INTERNAL_IO",
            "Internal I/O error happened",
            None,
        )
    }
}

use std::fmt;
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GeneralError [Stage: {} | Code: {} | Message: {} | Details: {}]",
            self.stage.as_ref().map(|x| &**x).unwrap_or("None"),
            self.code,
            self.message,
            self.details.as_ref().unwrap_or(&Value::Null)
        )
    }
}

use actix_web::dev::Body;
impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse<Body> {
        HttpResponse::build(self.status).json(json!({ "error": self }))
    }
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("General", 4)?;
        state.serialize_field("stage", &self.stage)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("details", &self.details)?;
        state.end()
    }
}

impl Into<HttpResponse> for ApiError {
    fn into(self) -> HttpResponse {
        HttpResponse::build(self.status).json(json!({ "error": self }))
    }
}

impl<E: Display + Serialize> From<MarkError<E>> for ApiError {
    fn from(err: MarkError<E>) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            Some("MARKUP"),
            "MARKUP_ERROR",
            "Markup error occured",
            Some(json!({ "err": err })),
        )
    }
}
