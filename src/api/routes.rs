use super:: { web, models::* };
use crate::SharedStorage;
use crate::storage::adapters::KeyExpiration;
use core::time::Duration;
use actix_web::{get, post, HttpResponse, Responder};


#[get("/load/{key}")]
async fn load(web::Path(key): web::Path<String>, data: web::Data<SharedStorage>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();
    let data = storage.get(&key);
    HttpResponse::Ok().json(LoadedData { data })
}


#[post("/save")]
async fn save(req_body: web::Json<SaveBody>, data: web::Data<SharedStorage>, query: web::Query<SaveQuery>) -> impl Responder {
    if query.ttl_min.is_some() && query.attempts.is_some() { return HttpResponse::BadRequest().body("Only one of ttl_min or attempts can be specified.") }
    
    let expiration = if let Some(ttl) = query.ttl_min { KeyExpiration::AfterTime(Duration::from_secs(ttl*60)) } 
                     else if let Some(attempts) = query.attempts { KeyExpiration::AfterUse(attempts) } 
                     else { KeyExpiration::Never };

    let mut storage = data.storage.lock().unwrap();
    let key = storage.save_and_expire(req_body.into_inner().value, expiration);
    HttpResponse::Ok().json(SavedKey { key })
}