use super::*;
use actix_web::{
    dev::{Body, ServiceResponse},
    error::Error as AcError,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Authorization;

impl<S> Transform<S> for Authorization
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<Body>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizationMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthorizationMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S> Service for AuthorizationMiddleware<S>
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

            let header = req.headers().get("Authorization").map(Clone::clone);

            req.extensions_mut()
                .insert::<Option<models::Token>>(match header {
                    Some(v) if !v.is_empty() => {
                        let auth = v.to_str().map_err(convert_err)?;

                        let uuid = Uuid::parse_str(auth)
                            .map_err(|_| ApiError::authorization_bad_token())?;

                        let (_, token) = models::Token::find_by_uuid(&uuid, conn)
                            .map_err(convert_err)
                            .await?;

                        Some(token.ok_or(ApiError::authorization_bad_token())?)
                    }
                    _ => None,
                });

            let req = ServiceRequest::from_parts(req, pl)
                .map_err(|_| "req is cloned!")
                .unwrap();
            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
