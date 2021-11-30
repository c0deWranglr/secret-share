use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Error};

#[get("/")]
pub async fn index() -> Result<NamedFile, Error> {
    let path = PathBuf::from("../client/build/index.html");

    let file = NamedFile::open(path)?;
    Ok(file)
}

#[get("/{filename:.*}")]
pub async fn static_content(req: HttpRequest) -> Result<NamedFile, Error> {
    let mut path = PathBuf::from("../client/build/");
    path.push::<PathBuf>(req.match_info().query("filename").parse().unwrap());
    
    let file = NamedFile::open(path.clone());
    if let Ok(file) = file {
        return Ok(file)
    } else {
        println!("Path {:?} not found, returning index", path);
        let path = PathBuf::from("../client/build/index.html");

        let file = NamedFile::open(path)?;
        Ok(file)
    }
}