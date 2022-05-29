use super::*;
use actix_web::http::header::{HeaderName, HeaderValue};
use rand::distributions::Alphanumeric;
use sha2::{Digest, Sha256};

#[get("dcaptcha.png")]
pub async fn dcaptcha_generate(app_state: Data<AppState>) -> Result<impl Responder, ApiError> {
    // generate image
    let (key, png) = app_state.gen_dcaptcha().await;

    // generate key prefix
    let mut rng = rand::thread_rng();
    let prefix: String = (0..config::DCAPTCHA_PREFIX_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0, config::DCAPTCHA_PREFIX_CHARSET.len());
            config::DCAPTCHA_PREFIX_CHARSET[idx] as char
        })
        .collect();

    // hash
    let mut hasher = Sha256::new();
    hasher.update(&prefix.as_bytes());
    hasher.update(&key);
    let hash_hex = format!("{:x}", &hasher.finalize());

    let mut resp = HttpResponse::build(StatusCode::OK);
    resp.set(actix_web::http::header::ContentType(mime::IMAGE_PNG));
    resp.header(
        HeaderName::from_static("x-dcaptcha-prefix"),
        HeaderValue::from_bytes(&prefix.to_string().as_bytes()).unwrap(),
    );
    resp.header(
        HeaderName::from_static("x-dcaptcha-hash"),
        HeaderValue::from_bytes(&hash_hex.as_bytes()).unwrap(),
    );
    resp.header(
        HeaderName::from_static("x-dcaptcha-lifetime"),
        HeaderValue::from_bytes(&config::DIGIT_CAPTCHA_LIFETIME.to_string().as_bytes()).unwrap(),
    );
    Ok(resp.body(png))
}
