use crate::utils::file_system;
use crate::error::Error;
use crate::utils::http::ParsedBody;
use std::path::PathBuf;
use std::str::FromStr;
use actix_web::{HttpResponse, Responder, Result};
use actix_web::web::{Path, Payload};
use futures::Future;

pub fn read_file(path: Path<(String,)>) -> Result<impl Responder> {
  let path = path.0.as_str();
  let path = file_system::get_full_path(PathBuf::from_str(path).unwrap());
  let contents = file_system::read_file(path?)?;

  Ok(HttpResponse::Ok()
    .content_type("text/plain")
    .body(contents))
}

pub fn create_file(path: Path<(String,)>, body: Payload) -> impl Future<Item = HttpResponse, Error = Error> {
  let path = path.0.as_str();
  let path = file_system::get_full_path(PathBuf::from_str(path).unwrap());

  ParsedBody::parse(body).and_then(|parsed| {
    let bytes = parsed.0.as_slice();
    let string_content = String::from_utf8_lossy(bytes);

    match file_system::create_file(path?, string_content.to_string()) {
      Ok(value) => Ok(HttpResponse::Ok().body(value)),
      Err(err) => Err(err)
    }
  })
}

pub fn update_file(path: Path<(String,)>, body: Payload) -> impl Future<Item = HttpResponse, Error = Error> {
  let path = path.0.as_str();
  let path = file_system::get_full_path(PathBuf::from_str(path).unwrap());

  ParsedBody::parse(body).and_then(|parsed| {
    let bytes = parsed.0.as_slice();
    let string_content = String::from_utf8_lossy(bytes);

    match file_system::update_file(path?, string_content.to_string()) {
      Ok(value) => Ok(HttpResponse::Ok().body(value)),
      Err(err) => Err(err)
    }
  })
}

pub fn delete_file(path: Path<(String,)>) -> Result<impl Responder> {
  let path = path.0.as_str();
  file_system::delete_file(PathBuf::from_str(path).unwrap())?;
  Ok(HttpResponse::Ok())
}