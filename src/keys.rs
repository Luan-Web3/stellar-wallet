use bip39::{Language, Mnemonic};
use ed25519_dalek::{SigningKey, VerifyingKey};
use hex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Keys {
    pub public_key: String,
    pub secret_key: String,
}

impl Keys {
    pub fn generate_mnemonic() -> String {
        let m = Mnemonic::generate_in(Language::English, 12).unwrap();
        m.words().collect::<Vec<&str>>().join(" ")
    }

    pub fn from_mnemonic(mnemonic: &str) -> Self {
        let mnemonic = Mnemonic::parse(mnemonic).expect("Invalid mnemonic");
        let seed = mnemonic.to_seed("");

        let seed_bytes = &seed[..32];

        let signing_key = SigningKey::from_bytes(seed_bytes.try_into().unwrap());
        let verifying_key: VerifyingKey = (&signing_key).into();

        Self {
            public_key: hex::encode(verifying_key.to_bytes()),
            secret_key: hex::encode(signing_key.to_bytes()),
        }
    }
}
