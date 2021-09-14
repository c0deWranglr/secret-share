use super:: { web, models::* };
use crate::SharedStorage;
use actix_web::{get, post, HttpResponse, Responder};


#[get("/load/{key}")]
async fn load(web::Path(key): web::Path<String>, data: web::Data<SharedStorage>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();
    let data = storage.get(&key);
    HttpResponse::Ok().json(LoadedData { data })
}


#[post("/save")]
async fn save(req_body: web::Json<SaveBody>, data: web::Data<SharedStorage>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();
    let key = storage.save(req_body.into_inner().value);
    HttpResponse::Ok().json(SavedKey { key })
}