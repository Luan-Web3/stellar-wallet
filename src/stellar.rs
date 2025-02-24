use crate::keys::Keys;
use anyhow::{Context, Error, Ok, Result};
use chrono::Duration;
use reqwest::Client;
use serde_json::Value;
use std::str::FromStr;
use stellar_base::amount::Amount;
use stellar_base::asset::Asset;
use stellar_base::crypto::{PublicKey, SodiumKeyPair};
use stellar_base::memo::Memo;
use stellar_base::network::Network;
use stellar_base::operations::Operation;
use stellar_base::time_bounds::TimeBounds;
use stellar_base::transaction::{Transaction, MIN_BASE_FEE};
use stellar_base::xdr::XDRSerialize;
// use stellar_base::{
//     amount::Amount,
//     crypto::KeyPair,
//     network::Network,
//     operations::Operation,
//     transaction::{Transaction, TransactionEnvelope},
//     Asset,
// };
pub struct Stellar {
    horizon_url: String,
}

impl Stellar {
    pub fn new(horizon_url: &str) -> Self {
        Self {
            horizon_url: horizon_url.to_string(),
        }
    }

    pub async fn get_balance(&self, account_id: &str) -> Result<Vec<Value>> {
        let client = Client::new();

        let url: String = format!("{}/accounts/{}", self.horizon_url, account_id);
        let response = client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch account: {}", response.status());
        }

        let account: Value = response.json().await.context("Failed to parse JSON")?;

        Ok(account["balances"].as_array().unwrap_or(&vec![]).to_vec())
    }

    pub async fn transfer_xlm(
        &self,
        source_secret: &str,
        destination_public: &str,
        amount: &str,
    ) -> Result<Vec<Value>> {
        println!("==============");
        let client = Client::new();

        let source_kp = Keys::get_public_key_from_private(source_secret)?;
        let destination = PublicKey::from_account_id(destination_public)?;
        println!("Destination {:?}", destination);

        let payment_amount = Amount::from_str(amount)?;

        let operation = Operation::new_payment()
            .with_destination(destination)
            .with_amount(payment_amount)?
            .with_asset(Asset::new_native())
            .build()?;
        let current_sequence =
            Self::get_account_sequence(&self, &source_kp.public_key().to_string()).await?;

        let next_sequence = current_sequence + 1;
        println!("PUBLIC KEY {}", source_kp.public_key());

        let time_bounds = TimeBounds::always_valid();

        let mut tx = Transaction::builder(source_kp.public_key(), next_sequence, MIN_BASE_FEE)
            .with_memo(Memo::new_none())
            .with_time_bounds(time_bounds)
            .add_operation(operation)
            .into_transaction()?;

        let _ = tx.sign(&source_kp.as_ref(), &Network::new_test());
        let xdr = tx.into_envelope().xdr_base64()?;

        let url = format!("{}/transactions", self.horizon_url);

        println!("XDR: {:?}", xdr);
        let response = client
            .post(&url)
            .form(&[("tx", xdr)])
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            println!("{:?}", response);
            anyhow::bail!("Failed to fetch account: {}", response.status());
        }

        let res_data: Value = response.json().await.context("Failed to parse JSON")?;
        println!("{:?}", res_data);
        Ok(res_data.as_array().unwrap_or(&vec![]).to_vec())
    }

    async fn get_account_sequence(&self, public_key: &str) -> Result<i64> {
        let url = format!("{}/accounts/{}", self.horizon_url, public_key);

        let response = reqwest::get(&url).await?;
        let account_data: Value = response.json().await?;

        println!("Account Data {:?}", account_data["sequence"]);

        // Extract sequence number and convert to i64
        let sequence = account_data["sequence"]
            .as_str()
            .ok_or(anyhow::anyhow!("No sequence found"))?
            .parse::<i64>()?;

        println!("seque: {:?}", sequence);

        Ok(sequence)
    }
}
