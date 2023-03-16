extern crate rand_core;
extern crate base64;
extern crate cloud_storage;
extern crate awc;

mod props;
mod storage;
mod api;
mod index;

use storage::{ Storage, adapters::{StorageAdapter, InMemoryHash, CloudStorage} };
use props::{PROPS, EnabledStorage};

use std::sync::Mutex;
use actix_web::{web, App, HttpServer};
use actix_server::Server;

type SharedStorage<A> = Mutex<Storage<A>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match &PROPS.storage {
        EnabledStorage::GCloud(props) => serve(CloudStorage::new(props)),
        EnabledStorage::InMemory => serve(InMemoryHash::new())
    }?.await
}

fn serve<A: StorageAdapter + Send + 'static>(adapter: A) -> std::io::Result<Server> {
    let storage: web::Data<SharedStorage<A>> = web::Data::new(Mutex::new(Storage::new(adapter)));

    let port = std::env::var("PORT").unwrap_or(String::from("8080"));

    let serv = HttpServer::new(move || {
        App::new()
        .wrap(actix_cors::Cors::permissive())
        .app_data(storage.clone())
        .service(web::scope("/api").configure(api::configure::<A>))
        .service(index::index)
        .service(index::static_content)
    })
    .bind(format!("0.0.0.0:{}", port))?;

    if cfg!(debug_assertions) {
        println!("Running server on http://127.0.0.1:{}", port);
    } else {
        println!("Running server")
    }

    Ok(serv.run())
}