use super:: { web, models::* };
use crate::SharedStorage;
use crate::storage::adapters::StorageAdapter;
use core::time::Duration;
use actix_web::{HttpResponse, Responder};

pub struct Routes<A: StorageAdapter> {
    a: std::marker::PhantomData<A>
}

impl<A: StorageAdapter> Routes<A> {
    pub async fn load(key: web::Path<String>, data: web::Data<SharedStorage<A>>) -> impl Responder {
        let mut storage = data.lock().unwrap();
        let data = storage.get(&key);
        if data.is_err() { 
            println!("Exceptional load result: {:?}", data);
            HttpResponse::NotFound().body("No data found for key")
        } else {
            HttpResponse::Ok().json(LoadedData { data: data.ok() })
        }
    }
    
    pub async fn save(req_body: web::Json<SaveBody>, data: web::Data<SharedStorage<A>>, query: web::Query<SaveQuery>) -> impl Responder {        
        let mut storage = data.lock().unwrap();
        let key = storage.save(req_body.into_inner().value, query.attempts, Duration::from_secs(query.ttl_min*60));
        if key.is_err() { 
            println!("Exceptional save result: {:?}", key) ;
            HttpResponse::InternalServerError().body("Error handling request")
        } else {
            HttpResponse::Ok().json(SavedKey { key: key.ok() })
        }
    }
}