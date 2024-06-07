use actix_web::{HttpResponse, Responder};

pub async fn hello<'a>(user: Option<&'a str>) -> impl Responder {
    HttpResponse::Ok().body(&format!("Hello {}!", user.unwrap_or("User")))
}
