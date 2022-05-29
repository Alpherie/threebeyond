use super::*;
use actix_web::client::Client;
use actix_web::dev::{Body, ServiceResponse};
use actix_web::http::HeaderMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Deserialize)]
pub struct HCaptchaResponse {
    pub success: bool,
}

pub struct Captcha {
    pub reply: bool,
}

impl<S> Transform<S> for Captcha
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<Body>;
    type Error = Error;
    type InitError = ();
    type Transform = CaptchaMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CaptchaMiddleware {
            service: Rc::new(RefCell::new(service)),
            reply: self.reply,
        })
    }
}

pub struct CaptchaMiddleware<S> {
    service: Rc<RefCell<S>>,
    reply: bool,
}

impl<S> Service for CaptchaMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<Body>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let (req, pl) = req.into_parts();
        let mut svc = self.service.clone();
        let reply = self.reply;

        Box::pin(async move {
            let exts = req.extensions();
            let token = exts
                .get::<Option<models::Token>>()
                .map(Clone::clone)
                .unwrap();
            drop(exts);

            if let None = token {
                macro_rules! get_header {
                    ($name:expr, $err:expr) => {
                        match match req.headers().get($name) {
                            Some(r) => r,
                            None => {
                                return Ok(ServiceResponse::new(req, $err().into()));
                            }
                        }
                        .to_str()
                        {
                            Ok(v) => v,
                            Err(e) => {
                                return Ok(ServiceResponse::new(req, ApiError::from(e).into()))
                            }
                        };
                    };
                }

                let kind = get_header!("X-Captcha-Kind", ApiError::captcha_kind_required);
                if !if kind == "op" {
                    reply
                } else {
                    let value = get_header!("X-Captcha-Value", ApiError::captcha_value_required);
                    match kind {
                        "hcaptcha" => {
                            Client::default()
                                .post(config::HCAPTCHA_URL)
                                .send_form(&json!({
                                    "response": value,
                                    "secret": config::HCAPTCHA_SECRET_KEY
                                }))
                                .await?
                                .json::<HCaptchaResponse>()
                                .await?
                                .success
                        }

                        "dcaptcha" => {
                            web::Data::<AppState>::extract(&req)
                                .await?
                                .validate_dcaptcha(value)
                                .await
                        }

                        _ => {
                            return Ok(ServiceResponse::new(
                                req,
                                ApiError::captcha_kind_unsupported().into(),
                            ))
                        }
                    }
                } {
                    return Ok(ServiceResponse::new(
                        req,
                        ApiError::captcha_value_invalid().into(),
                    ));
                };
            }

            let req = ServiceRequest::from_parts(req, pl)
                .map_err(|_| "req is cloned!")
                .unwrap();
            svc.call(req).await
        })
    }
}
