use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::{Error, ErrorUnauthorized};
use std::future::{Ready, ready};
use std::result::Result;
use actix_web::{HttpMessage};
use futures_util::future::LocalBoxFuture;
use crate::props::PROPS;

pub struct Validation;

impl<S, B> Transform<S, ServiceRequest> for Validation
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidationMiddleware { service }))
    }
}

pub struct ValidationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ValidationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let captcha_header = req.get_header::<super::web::XCaptchaToken>();
        let service_call = self.service.call(req);
        Box::pin(async {
            if let Some(captcha_header) = captcha_header {
                let mut res = awc::Client::default()
                                      .post("https://hcaptcha.com/siteverify")
                                      .send_form(&[
                                          ("response", &captcha_header.token), 
                                          ("secret", &PROPS.hcaptcha.secret), 
                                          ("sitekey", &PROPS.hcaptcha.site_key)
                                       ])
                                      .await
                                      .map_err(|e| { eprintln!("Failed to validate captcha: {}", e); ErrorUnauthorized("Failed to validate captcha") })?;
                let res_body: super::web::CaptchaResponse = serde_json::from_slice(&res.body().await?)?;
                if !res.status().is_success() || !res_body.success {
                    eprintln!("Failed to validate captcha: {:?}", res_body);
                    return Err(ErrorUnauthorized("Failed to validate captcha response"));
                }
            }

            service_call.await
        })
    }
}