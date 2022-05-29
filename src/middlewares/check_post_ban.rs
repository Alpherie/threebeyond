use super::*;
use actix_web::{
    dev::{Body, ServiceResponse},
    error::Error as AcError,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct CheckPostBan;

impl<S> Transform<S> for CheckPostBan
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<Body>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckPostBanMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckPostBanMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct CheckPostBanMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S> Service for CheckPostBanMiddleware<S>
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

        fn convert_err<E>(source: E) -> AcError
        where
            E: Into<ApiError>,
        {
            AcError::from(source.into())
        }

        Box::pin(async move {
            let pool = web::Data::<Pool>::extract(&req).await?;
            let conn = pool.get_conn().await.map_err(convert_err)?;
            // get ip
            let connection_info = req.connection_info();

            //in future: realip_remote_addr
            let realip = connection_info.remote();
            let mut ip = match realip {
                Some(v) => match v.parse() {
                    Ok(t) => t,
                    Err(_) => req.peer_addr().unwrap().ip(),
                },

                None => req.peer_addr().unwrap().ip(),
            };
            drop(connection_info);
            let prep: models::BanIpAddress = ip.into();
            let p = web::Path::<(String, String)>::extract(&req).await?;

            // check if token present
            let exts = req.extensions();
            let t = exts.get::<Option<models::Token>>().map(Clone::clone);
            drop(exts);

            match t {
                Some(Some(token)) => {
                    let (conn, ban) = models::TokenBan::find_by_token_id(token.id, conn)
                        .await
                        .map_err(convert_err)?;

                    if let Some(info) = ban {
                        let (_, reason) =
                            //FIXME: doble
                            models::BanReason::find(info.reason, conn)
                                .await
                                .map(|(c, v)| {
                                    (
                                        c,
                                        v.expect(&format!("Unknown ban reason id {}", info.reason)),
                                    )
                                }).map_err(convert_err)?;

                        let err = ApiError::token_ban(info, reason).into();

                        // Ok instead of Err for other middlewares like CORS
                        return Ok(ServiceResponse::new(req, err));
                    }
                }

                _ => {
                    // check by ip
                    let (conn, ban) =
                        models::Ban::find_wn_lang_wn_board_first(prep, &p.0, &p.1, conn)
                            .await
                            .map_err(convert_err)?;

                    if let Some(info) = ban {
                        let (_, reason) =
                            //FIXME: doble
                            models::BanReason::find(info.reason, conn)
                                .await
                                .map(|(c, v)| {
                                    (
                                        c,
                                        v.expect(&format!("Unknown ban reason id {}", info.reason)),
                                    )
                                }).map_err(convert_err)?;

                        let err = ApiError::ban(info, reason).into();

                        // Ok instead of Err for other middlewares like CORS
                        return Ok(ServiceResponse::new(req, err));
                    }
                }
            }

            let req = ServiceRequest::from_parts(req, pl)
                .map_err(|_| "req is cloned!")
                .unwrap();
            let call = svc.call(req);
            let res = call.await?;
            Ok(res)
        })
    }
}
