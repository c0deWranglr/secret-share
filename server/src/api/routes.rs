use super:: { web, models::* };
use crate::SharedStorage;
use core::time::Duration;
use actix_web::{get, post, HttpResponse, Responder};

#[get("/load/{key}")]
async fn load(web::Path(key): web::Path<String>, data: web::Data<SharedStorage>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();
    let data = storage.get(&key);
    if let Err(error) = &data {
        println!("Failed to load data: {:?}", error);
    }
    HttpResponse::Ok().json(LoadedData { data: data.ok() })
}

#[post("/save")]
async fn save(req_body: web::Json<SaveBody>, data: web::Data<SharedStorage>, query: web::Query<SaveQuery>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();
    let key = storage.save(req_body.into_inner().value, query.attempts, Duration::from_secs(query.ttl_min*60));
    if let Err(error) = &key { 
        println!("Failed to save data: {:?}", error); 
    }
    HttpResponse::Ok().json(SavedKey { key: key.ok() })
}