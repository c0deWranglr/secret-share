pub mod models;
mod routes;
mod middleware;

pub use routes::Routes;

use crate::storage::adapters::StorageAdapter;
use actix_web::{web};

pub fn configure<A: StorageAdapter + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/load/{key}")
                    .wrap(middleware::captcha::Validation)
                    .route(web::get()
                               .to(Routes::<A>::load)));
    cfg.service(web::resource("/save")
                    .wrap(middleware::captcha::Validation)
                    .route(web::post()
                               .to(Routes::<A>::save)));
}