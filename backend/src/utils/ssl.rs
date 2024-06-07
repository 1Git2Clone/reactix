use std::{fs::File, io::Write, path::Path};

use openssl::{
    asn1::Asn1Time, hash::MessageDigest, pkey, pkey::PKey, rsa::Rsa, x509,
    x509::extension::BasicConstraints,
};

use crate::data::cert::{CERT_KEY_PEM, CERT_PEM, NOPASS_CERT_KEY_PEM};

/// Creates a X509 Certificate file and a PEM key file for it as well as an unencrypted nopass.pem
/// file.
///
/// **WARNING:** Storing the unencrypted file is bad OPSEC and is disencouraged.
///
/// The shell equivalent of this would be
///
/// ```sh
/// # NOTE: RSA4096 encryption for the certificate and key with sha256 signing and a country entry
/// # with a 1 year (365 day) duration.
/// openssl req -x509 -newkey rsa:4096 -keyout CERT_KEY_PEM -out CERT_PEM \
/// -days 365 -sha256 -subj "/C=BG" \
///   -passout pass:$SSL_PASSWORD
///
/// # WARNING: Unencrypting to the NOPASS_CERT_KEY_PEM file.
/// openssl rsa -in CERT_KEY_PEM -out NOPASS_CERT_KEY_PEM -passin pass:$SSL_PASSWORD
/// ```
///
/// For more info you can also refer to this actix documentation:
/// <https://actix.rs/docs/server#tls--https>
pub fn manage_certs_files(password: String, store_unencrypted: bool) -> std::io::Result<()> {
    let files = if store_unencrypted {
        vec![CERT_PEM, CERT_KEY_PEM, NOPASS_CERT_KEY_PEM]
    } else {
        vec![CERT_PEM, CERT_KEY_PEM]
    };

    // If there's some file missing...
    if !files
        .iter()
        .filter(|x| !Path::new(x).exists())
        .collect::<Vec<_>>()
        .is_empty()
    {
        let rsa: Rsa<pkey::Private> = Rsa::generate(4096)?;

        let pkey: PKey<pkey::Private> = PKey::from_rsa(rsa.clone())?;

        // Create X509 name
        let mut name = x509::X509NameBuilder::new()?;
        name.append_entry_by_text("C", "BG")?;
        let name = name.build();

        // Create X509 certificate
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

        // Write encrypted private key to file
        let mut key_file = File::create(CERT_KEY_PEM)?;
        let encrypted_key = rsa.private_key_to_pem_passphrase(
            openssl::symm::Cipher::aes_256_cbc(),
            password.as_bytes(),
        )?;
        key_file.write_all(&encrypted_key)?;

        // Write certificate to file
        let mut cert_file = File::create(CERT_PEM)?;
        cert_file.write_all(&cert.to_pem()?)?;

        if store_unencrypted {
            // Write unencrypted private key to file
            // WARNING: Not a safe practice for production deployments.
            let mut nopass_file = File::create(NOPASS_CERT_KEY_PEM)?;
            let unencrypted_key = rsa.private_key_to_pem()?;
            nopass_file.write_all(&unencrypted_key)?;
        }
    };

    Ok(())
}
