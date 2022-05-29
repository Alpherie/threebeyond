use super::*;
use actix_web::FromRequest;

pub struct Extension<T>(pub T);

impl<T: 'static + Clone> FromRequest for Extension<T> {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> <Self as actix_web::FromRequest>::Future {
        let t = req
            .extensions()
            .get::<T>()
            .map(Clone::clone)
            .expect("Unregistered extension");

        Box::pin(async { Ok(Self(t)) })
    }
}

impl<T> AsRef<T> for Extension<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Extension<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
