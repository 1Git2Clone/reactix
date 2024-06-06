use actix_web::{web, App, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslMethod};

mod api;
use api::api_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // NOTE: This is the reccomended way to implement this, however it requires you entering that
    // SSL certificate password every time.
    //
    //builder
    //    .set_private_key_file("cert/key.pem", SslFiletype::PEM)
    //    .unwrap();
    //builder.set_certificate_chain_file("cert/cert.pem").unwrap();

    // WARNING: You should use a password (`builder.set_private_key_file()` - refer to the note
    // above) for production deploys.
    builder.set_certificate_chain_file("cert/cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(api_configuration))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind_openssl(("127.0.0.1", 4664), builder)?
    .run()
    .await
}
