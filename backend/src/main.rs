use actix_web::{web, App, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslMethod};

mod api;
use api::api_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    // NOTE: This is the reccomended way to implement this, however it requires you entering that
    // SSL certificate password every time.
    //
    builder.set_private_key_file("cert/key.pem", openssl::ssl::SslFiletype::PEM)?;
    builder.set_certificate_chain_file("cert/cert.pem")?;

    // WARNING: You should use a password (`builder.set_private_key_file()` - refer to the note
    // above) for production deploys.
    //
    //builder.set_certificate_chain_file("cert/cert.pem")?;

    HttpServer::new(|| {
        App::new().service(web::scope("/api").configure(api_configuration))
        // NOTE: This route is for debugging purposes.
        //
        //.route(
        //    "/",
        //    web::get().to(|| async { HttpResponse::Ok().body("Server running.") }),
        //)
    })
    // It's an `https://` page. Browsers will yell at you because it's a self-signed SSL certificate
    // though.
    .bind_openssl(("127.0.0.1", 16600), builder)?
    .run()
    .await
}
