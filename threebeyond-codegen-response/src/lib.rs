#[macro_export]
macro_rules! json {
	($code:ident, $obj:expr) => {
        Ok(actix_web::HttpResponse::build(actix_web::http::StatusCode::$code).json(json!({
        	"response": $obj
        })))
    };

    ($obj:expr) => {
        Ok(actix_web::HttpResponse::build(actix_web::http::StatusCode::OK).json(json!({
        	"response": $obj
        })))
    };
}

#[macro_export]
macro_rules! no {
	() => {
        Ok(actix_web::HttpResponse::build(actix_web::http::StatusCode::NO_CONTENT))
    };
}
