pub const CERT_PEM: &str = "cert/cert.pem";

// NOTE: This is better as it's an encrypted key. However this means that you'll have to manually
// enter the password each time you run the server.

pub const CERT_KEY_PEM: &str = "cert/key.pem";

// WARNING:
// This `nopass.pem` file (as the name implies) doesn't have the additional password encryption.
// This is convenient in development but discouraged in production.

pub const NOPASS_CERT_KEY_PEM: &str = "cert/nopass.pem";
