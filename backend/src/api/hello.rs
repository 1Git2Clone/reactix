use actix_web::{get, http::header::ContentType, HttpRequest, HttpResponse, Responder, Result};

#[get("/api/hello")]
pub async fn hello(_req: HttpRequest) -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Hello!"))
}

#[get("/api/hello/")]
pub async fn hello_no_user(_req: HttpRequest) -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Enter a name!"))
}

#[derive(serde::Serialize)]
struct User {
    name: String,
}

#[get("/api/hello/{username}")]
pub async fn hello_user(username: actix_web::web::Path<String>) -> Result<impl Responder> {
    let user = User {
        name: username.to_string(),
    };
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Hello {}!", user.name)))
}
