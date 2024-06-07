use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use openssl::{
    asn1::Asn1Time,
    hash::MessageDigest,
    pkey::{self, PKey, Private},
    rsa::Rsa,
    x509::{self, extension::BasicConstraints},
};

use crate::data::cert::{CERT_KEY_PEM, CERT_PEM};

/// Creates a X509 Certificate file and a PEM key file or uses their private key if the files
/// already exist. By returning the private key you can set it in the main function and the key
/// will be set without requiring any additional runtime user input. As long as your environment
/// variable key is correct.
///
/// For more info you can also refer to this actix documentation:
/// <https://actix.rs/docs/server#tls--https>
pub fn manage_certs_files(password: String) -> std::io::Result<PKey<Private>> {
    let files = [CERT_PEM, CERT_KEY_PEM].map(|x| Path::new(x));

    let separator = "-".repeat(80);

    if files
        .iter()
        .filter(|x| !x.exists())
        .collect::<Vec<_>>()
        .is_empty()
    {
        println!(
            "{0}\n{1} and {2} files already exist. Using their private key.\n{0}",
            &separator, CERT_PEM, CERT_KEY_PEM
        );
        let key_data: Vec<u8> = {
            let mut buffer = Vec::new();
            File::open(CERT_KEY_PEM)?.read_to_end(&mut buffer)?;
            buffer
        };
        let rsa = Rsa::private_key_from_pem_passphrase(&key_data, password.as_bytes())?;
        let pkey = PKey::from_rsa(rsa)?;
        return Ok(pkey);
    }

    println!(
        "{0}\nMaking {1} and {2} files.\n{0}",
        &separator, CERT_PEM, CERT_KEY_PEM
    );

    println!("Generating private RSA and Pkey.");
    let rsa: Rsa<pkey::Private> = Rsa::generate(4096)?;
    let pkey: PKey<pkey::Private> = PKey::from_rsa(rsa.clone())?;
    println!("Done.");

    println!("Creating X509 name.");
    let mut name = x509::X509NameBuilder::new()?;
    name.append_entry_by_text("C", "BG")?;
    let name = name.build();
    println!("Done.");

    println!("Creating X509 certificate.");
    let mut builder = x509::X509Builder::new()?;
    builder.set_version(2)?;
    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;
    builder.set_pubkey(&pkey)?;
    builder.set_not_before(&Asn1Time::days_from_now(0)?.as_ref())?;
    builder.set_not_after(&Asn1Time::days_from_now(365)?.as_ref())?;
    builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;
    builder.sign(&pkey, MessageDigest::sha256())?;
    let cert = builder.build();
    println!("Done.");

    println!("Writing to {}.", CERT_KEY_PEM);
    let mut key_file = File::create(CERT_KEY_PEM)?;
    let encrypted_key = rsa
        .private_key_to_pem_passphrase(openssl::symm::Cipher::aes_256_cbc(), password.as_bytes())?;
    key_file.write_all(&encrypted_key)?;
    println!("Done.");

    println!("Writing to {}.", CERT_PEM);
    let mut cert_file = File::create(CERT_PEM)?;
    cert_file.write_all(&cert.to_pem()?)?;
    println!("Done.");

    Ok(pkey)
}
