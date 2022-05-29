use super::*;

pub fn ok_json<T: serde::Serialize>(code: StatusCode, t: T) -> HttpResponse {
    HttpResponse::build(code)
        .content_type("application/json")
        .json(json!({ "response": t }))
}

pub fn error_json<T: serde::Serialize>(code: StatusCode, t: T) -> HttpResponse {
    HttpResponse::build(code)
        .content_type("application/json")
        .json(json!({ "error": t }))
}
