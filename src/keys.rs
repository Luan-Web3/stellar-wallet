use serde::Deserialize;
use std::fmt;
use std::result::Result::Ok;
use stellar_base::crypto::SodiumKeyPair;

#[derive(Debug)]
pub struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MyError {}

#[derive(Deserialize, Debug)]
pub struct Keys {}

impl Keys {
    pub fn generate_stellar_keys() -> Result<(String, String), MyError> {
        match SodiumKeyPair::random() {
            Ok(keypair) => Self::extract_keys_from_keypair(keypair),
            Err(e) => Err(MyError(format!("Failed to generate keys: {}", e))),
        }
    }

    pub fn get_public_key_from_private(secret_key: &str) -> Result<SodiumKeyPair, MyError> {
        match SodiumKeyPair::from_secret_seed(secret_key) {
            Ok(keypair) => Ok(keypair),
            Err(e) => Err(MyError(format!("Failed to generate keys: {}", e))),
        }
    }

    fn extract_keys_from_keypair(keypair: SodiumKeyPair) -> Result<(String, String), MyError> {
        let public_key = keypair.public_key().account_id();
        let secret_key = keypair.secret_key().secret_seed();
        Ok((public_key, secret_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[test]
    fn test_generate_stellar_keys() {
        let (public_key, private_key) = Keys::generate_stellar_keys().unwrap();
        assert!(public_key.starts_with('G'));
        assert!(private_key.starts_with('S'));
    }

    #[test]
    fn test_decode_strkey_stellar_lab() {
        dotenv().ok();
        let stellar_lab_secret = &std::env::var("SECRET_ID").unwrap();
        let result = Keys::get_public_key_from_private(stellar_lab_secret);
        assert!(
            result.is_ok(),
            "Falha ao decodificar a chave do Stellar Lab: {:?}",
            result
        );
    }
}
