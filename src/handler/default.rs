use actix_web::{HttpResponse, HttpRequest, Responder};

pub fn not_found(req: HttpRequest) -> impl Responder {
  HttpResponse::Ok().body(&format!("Not found: {}", req.uri()))
}