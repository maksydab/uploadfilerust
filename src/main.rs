#[macro_use] 
extern crate rocket;
use rocket::response::content;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Read, Error};
use rocket::fs::NamedFile;
use std::path::PathBuf;
use std::io;
use rand::prelude::*;
use rocket::fs::TempFile;
use rocket::form::Form;
use rocket::http::RawStr;
use rocket::tokio::fs;
use std::path::Path;
use rocket::data::{ByteUnit, ToByteUnit};

mod helpers;
mod frontend;

//█████████████████████████████████████
//█▄─▄▄▀█─▄▄─█▄─██─▄█─▄─▄─█▄─▄▄─█─▄▄▄▄█
//██─▄─▄█─██─██─██─████─████─▄█▀█▄▄▄▄─█
//█▄▄█▄▄█▄▄▄▄██▄▄▄▄███▄▄▄██▄▄▄▄▄█▄▄▄▄▄█
#[get("/name")]
fn namer()-> String{
    let n = helpers::randomid();
    n
}
#[get("/file/<path..>")]
async fn servefile(path: PathBuf) -> io::Result<NamedFile> {
    let mut full_path = std::path::PathBuf::from("files/");
    full_path.push(path);
    NamedFile::open(full_path).await
}

#[post("/upload/<filename>", data = "<data>")]
async fn upload(filename: &str, data: rocket::Data<'_>) -> String {
    let id = helpers::randomid();
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");
    let path = format!("files/{}.{}", id, ext);

    fs::create_dir_all("files").await.unwrap();
    let mut file = fs::File::create(&path).await.unwrap();
    rocket::tokio::io::copy(&mut data.open(ByteUnit::kibibytes(ByteUnit::Byte(100 * 1024 * 1024))), &mut file).await.unwrap();

    path
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![frontend::hello,frontend::ihateyou]).mount("/api", routes![servefile,namer,upload]).register("/", catchers![frontend::not_found])
}
