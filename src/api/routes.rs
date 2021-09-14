use super:: { web, models::* };
use crate::SharedStorage;
use actix_web::{post, HttpResponse, Responder};

#[post("/save")]
async fn save(req_body: web::Json<SaveBody>, data: web::Data<SharedStorage>) -> impl Responder {
    let key = data.storage.lock().unwrap().save(req_body.into_inner().value);
    HttpResponse::Ok().json(SavedKey { key })
}