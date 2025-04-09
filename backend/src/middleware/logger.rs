use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::dev::{Service, Transform};
use futures::future::{ok, Ready};
use futures::Future;

pub struct Logger;

impl Logger {
    pub fn new() -> Logger {
        log4rs::init_file("log4rs.yml", Default::default()).unwrap_or(());
        Logger {}
    }
}

impl<S, B> Transform<S> for Logger
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggerMiddleware { service })
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service for LoggerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }
    fn call(&mut self, request: ServiceRequest) -> Self::Future {
        let method = request.method().to_string();
        let path = request.path().to_string();

        let fut = self.service.call(request);

        Box::pin(async move {
            let response = fut.await?;

            log::debug!(r#"{} {} "{}". "#, method, response.status(), path);

            Ok(response)
        })
    }
}