pub mod models;
mod routes;

pub use routes::Routes;

use crate::storage::adapters::StorageAdapter;
use actix_web::{web};

pub fn configure<A: StorageAdapter + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/load/{key}")
                    .route(web::get()
                               .to(Routes::<A>::load)));
    cfg.service(web::resource("/save")
                    .route(web::post()
                               .to(Routes::<A>::save)));
}