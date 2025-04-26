use actix_web::{web, HttpResponse};
use serde::Deserialize;

// https://www.joshmcguigan.com/blog/understanding-serde/
#[derive(Deserialize)]
pub struct Info {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<Info>) -> HttpResponse {
    HttpResponse::Ok().finish()
}