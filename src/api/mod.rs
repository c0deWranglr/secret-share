pub mod models;
pub mod routes;

use actix_web::{web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::load);
    cfg.service(routes::save);
}