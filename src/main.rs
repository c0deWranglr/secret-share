extern crate rand_core;
extern crate base64;

mod storage;
mod api;

use storage::{ Storage, adapters::{InMemoryHash} };

use std::sync::Mutex;
use actix_web::{web, App, HttpServer};

struct SharedStorage {
    storage: Mutex<Storage<InMemoryHash>>
}

impl SharedStorage {
    fn new() -> SharedStorage {
        SharedStorage { storage: Mutex::new(Storage::new(InMemoryHash::new())) }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = web::Data::new(SharedStorage::new());

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .service(web::scope("/api").configure(api::configure))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}