extern crate rand_core;
extern crate base64;

mod props;
mod storage;
mod api;

use storage::{ Storage, adapters::{InMemoryHash} };

use std::sync::Mutex;
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{web, get, App, HttpServer, HttpRequest, Error};

struct SharedStorage {
    storage: Mutex<Storage<InMemoryHash>>
}

impl SharedStorage {
    fn new() -> SharedStorage {
        SharedStorage { storage: Mutex::new(Storage::new(InMemoryHash::new())) }
    }
}

#[get("/")]
async fn index() -> Result<NamedFile, Error> {
    let path = PathBuf::from("../client/build/index.html");

    let file = NamedFile::open(path)?;
    Ok(file)
}

#[get("/{filename:.*}")]
async fn website_content(req: HttpRequest) -> Result<NamedFile, Error> {
    let mut path = PathBuf::from("../client/build/");
    path.push::<PathBuf>(req.match_info().query("filename").parse().unwrap());
    println!("Looking for {:?}", path);

    let file = NamedFile::open(path);
    if let Ok(file) = file {
        return Ok(file)
    } else {
        println!("Path not found, returning index");
        let path = PathBuf::from("../client/build/index.html");

        let file = NamedFile::open(path)?;
        Ok(file)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = web::Data::new(SharedStorage::new());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive())
            .app_data(storage.clone())
            .service(web::scope("/api").configure(api::configure))
            .service(index)
            .service(website_content)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}