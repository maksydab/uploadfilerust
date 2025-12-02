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

#[get("/")]
pub fn hello() -> Result<content::RawHtml<String>, Error> {
    Ok(content::RawHtml(helpers::returnhtml("src/frontend/index.html")))
}
#[catch(404)]
pub fn not_found() -> Result<content::RawHtml<String>, Error>  {
    Ok(content::RawHtml(helpers::returnhtml("src/frontend/404.html")))
}

#[get("/secondpage")]
pub fn ihateyou() -> Result<content::RawHtml<String>, Error> {
    Ok(content::RawHtml(helpers::returnhtml("src/frontend/second.html")))
}
