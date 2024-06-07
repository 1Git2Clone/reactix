pub mod api;
pub mod data;
pub mod utils;

use api::api_configuration;
#[allow(unused_imports)]
use data::cert::{CERT_KEY_PEM, CERT_PEM, NOPASS_CERT_KEY_PEM};
use utils::ssl;

use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let password = std::env::var("SSL_PASSWORD").expect("SSL_PASSWORD not set in `backend/.env`");
    let store_unencrypted = std::env::var("STORE_UNENCRYPTED")
        .unwrap_or(String::from("false"))
        .to_lowercase()
        .replace('\'', "")
        .replace('"', "");
    let store_unencrypted = if store_unencrypted == "true" {
        true
    } else {
        false
    };
    ssl::manage_certs_files(password, store_unencrypted)?;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    if store_unencrypted {
        // WARNING: It's not reccomended to store a nopass file at all for production!
        builder.set_private_key_file(NOPASS_CERT_KEY_PEM, openssl::ssl::SslFiletype::PEM)?;
    } else {
        // NOTE: This is the reccomended way to implement this, however it requires you entering that
        // SSL certificate password every time.
        builder.set_private_key_file(CERT_KEY_PEM, openssl::ssl::SslFiletype::PEM)?;
    }

    builder.set_certificate_chain_file(CERT_PEM)?;

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(api_configuration))
            // NOTE: This route is for debugging purposes.
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("[200]: Server running.") }),
            )
    })
    // It's an `https://` page.
    //
    // Browsers will yell at you because it's a self-signed SSL certificate
    // though.
    .bind_openssl(("127.0.0.1", 16600), builder)?
    .run()
    .await
}
