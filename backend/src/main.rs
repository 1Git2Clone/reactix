pub mod api;
pub mod data;
pub mod utils;

use api::api_configuration;
#[allow(unused_imports)]
use data::cert::{CERT_KEY_PEM, CERT_PEM};
use utils::ssl;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let password = std::env::var("SSL_PASSWORD").expect("SSL_PASSWORD not set in `backend/.env`");
    let pkey = ssl::manage_certs_files(password)?;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key(pkey.as_ref())?;
    builder.set_certificate_chain_file(CERT_PEM)?;

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("/", web::get().to(HttpResponse::Ok))
            .service(api::hello::hello)
            .service(api::hello::hello_user)
            .service(web::scope("/api").configure(api_configuration))
    })
    // It's an `https://` page.
    //
    // Browsers will yell at you because it's a self-signed SSL certificate
    // though.
    .bind_openssl(("127.0.0.1", 16600), builder)?
    .run()
    .await
}
