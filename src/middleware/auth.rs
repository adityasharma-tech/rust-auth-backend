use std::sync::{Arc, Mutex};
use actix_web::{middleware::{Compress, Logger}, web, App, HttpServer, HttpResponse, Error, HttpRequest};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use chrono::{DateTime, Utc};
use std::{future::{ready, Ready, Future}, pin::Pin};


// Here middleware is a struct
pub struct AuditDataBase;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuditDataBase
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MyAuditDataMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // Return a Ready future containing the AuditDataMiddleware instance
        ready(Ok(MyAuditDataMiddleware { service }))
    }
}

pub struct MyAuditDataMiddleware<S> {
    /// The next service to call after this one
    service: S,
}

impl<S, B> Service<ServiceRequest> for MyAuditDataMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    actix_web::dev::forward_ready!(service);
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let mut headers = req.headers_mut();
        headers.insert("user-name".parse().unwrap(), "sarath Raj".parse().unwrap()); // Add custom header
        headers.insert("timestamp".parse().unwrap(), format!("{}", Utc::now()).parse().unwrap()); // Add timestamp header
        let  future = self.service.call(req);
        Box::pin(async move {
            let res = match future.await {
                Ok(response) => response,
                Err(error) => panic!("Unable to process middleware: {}", error),
            };
            Ok(res)
        })
    }
}