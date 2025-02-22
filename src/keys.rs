use anyhow::Error;
use serde::Deserialize;
use stellar_sdk::Keypair;

#[derive(Deserialize, Debug)]
pub struct Keys {}

impl Keys {
    fn extract_keys_from_keypair(mut keypair: Keypair) -> Result<(String, String), Error> {
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key()?;
        Ok((public_key, secret_key))
    }

    pub fn generate_stellar_keys() -> Result<(String, String), Error> {
        let keypair = Keypair::random()?;
        Self::extract_keys_from_keypair(keypair)
    }

    pub fn get_public_key_from_private(secret_key: &str) -> Result<(String, String), Error> {
        let keypair = Keypair::from_secret_key(secret_key)?;
        Self::extract_keys_from_keypair(keypair)
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
