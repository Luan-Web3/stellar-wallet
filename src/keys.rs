use base32::{decode, encode, Alphabet};
use crc::{Crc, CRC_16_XMODEM};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Keys {}

const PUBLIC_KEY_VERSION_BYTE: u8 = 6 << 3;
const SECRET_KEY_VERSION_BYTE: u8 = 18 << 3;

impl Keys {
    fn compute_checksum(data: &[u8]) -> [u8; 2] {
        const CRC16_XMODEM: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);

        let checksum = CRC16_XMODEM.checksum(data);

        [(checksum >> 8) as u8, (checksum & 0xFF) as u8]
    }

    fn encode_strkey(data: &[u8], version_byte: u8) -> String {
        let mut payload = Vec::with_capacity(data.len() + 1);
        payload.push(version_byte);
        payload.extend_from_slice(data);

        let checksum = Self::compute_checksum(&payload);

        payload.extend_from_slice(&checksum);

        encode(Alphabet::Rfc4648 { padding: false }, &payload)
    }

    pub fn decode_strkey(key: &str) -> Result<Vec<u8>, String> {
        let decoded =
            decode(Alphabet::Rfc4648 { padding: false }, key).ok_or("Invalid Base32 encoding")?;

        if decoded.len() < 3 {
            return Err("Decoded data is too short".to_string());
        }

        let payload = &decoded[..decoded.len() - 2];
        let checksum = &decoded[decoded.len() - 2..];

        let expected_checksum = Self::compute_checksum(payload);
        if checksum != expected_checksum {
            return Err("Checksum verification failed".to_string());
        }

        Ok(payload[1..].to_vec())
    }

    pub fn generate_stellar_keys() -> Result<(String, String), Box<dyn Error>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);

        let public_key = signing_key.verifying_key();

        let public_strkey = Self::encode_strkey(public_key.as_bytes(), PUBLIC_KEY_VERSION_BYTE);
        let private_strkey = Self::encode_strkey(signing_key.as_bytes(), SECRET_KEY_VERSION_BYTE);

        Ok((public_strkey, private_strkey))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_stellar_keys() {
        let (public_key, private_key) = Keys::generate_stellar_keys().unwrap();
        assert!(public_key.starts_with('G'));
        assert!(private_key.starts_with('S'));
    }
}
