use super::*;
use actix_web::{
    dev::{Body, ServiceResponse},
    error::Error as AcError,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct PostTimeoutCheck;

impl<S> Transform<S> for PostTimeoutCheck
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<Body>;
    type Error = Error;
    type InitError = ();
    type Transform = PostTimeoutCheckMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(PostTimeoutCheckMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct PostTimeoutCheckMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S> Service for PostTimeoutCheckMiddleware<S>
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
            let state = web::Data::<AppState>::extract(&req).await?;
            let connection_info = req.connection_info();

            let realip = connection_info.remote();
            let mut ip = match realip {
                Some(v) => match v.parse() {
                    Ok(t) => t,
                    Err(_) => req.peer_addr().unwrap().ip(),
                },

                None => req.peer_addr().unwrap().ip(),
            };
            drop(connection_info);

            if let Some(instant) = state.last_posts.lock().await.get(&ip) {
                let mi = instant.elapsed().as_millis();
                if mi < config::POST_TIMEOUT {
                    let err = ApiError::last_post_timeout(config::POST_TIMEOUT - mi).into();
                    return Ok(ServiceResponse::new(req, err));
                }
            }

            let req = ServiceRequest::from_parts(req, pl)
                .map_err(|_| "req is cloned!")
                .unwrap();
            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
