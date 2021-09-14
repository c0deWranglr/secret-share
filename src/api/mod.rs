pub mod models;
pub mod routes;

use actix_web::{web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::save);
}