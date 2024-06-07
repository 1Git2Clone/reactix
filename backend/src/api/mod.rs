pub mod hello;

pub fn api_configuration(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::{web, HttpResponse};

    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(|| async { HttpResponse::MethodNotAllowed() })),
    );
}
