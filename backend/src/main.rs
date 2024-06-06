use actix_web::{
    web::{self, get},
    App, HttpResponse, HttpServer,
};
use openssl::{
    pkey::PKeyRef,
    ssl::{SslAcceptor, SslFiletype, SslMethod},
};

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

    // WARNING: You shouldn't store unencrypted SSL certificates (or any private info in this
    // unencrypted manner).
    builder.set_certificate_chain_file("cert/cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(api_configuration))
            .route("/", get().to(|| async { HttpResponse::Ok().body("/") }))
    })
    .bind_openssl(("127.0.0.1", 4664), builder)?
    .run()
    .await
}
