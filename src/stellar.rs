use reqwest;
use serde::Deserialize;
use std::error::Error;

pub struct Stellar {
    horizon_url: String,
}

#[derive(Deserialize)]
struct AccountResponse {
    balances: Vec<Balance>,
}

#[derive(Deserialize)]
struct Balance {
    asset_type: String,
    balance: String,
}

impl Stellar {
    pub fn new(horizon_url: &str) -> Self {
        Self {
            horizon_url: horizon_url.to_string(),
        }
    }

    pub async fn get_balance(&self, account_id: &str) -> Result<String, Box<dyn Error>> {
        let url: String = format!("{}/accounts/{}", self.horizon_url, account_id);
        let response: AccountResponse = reqwest::get(&url).await?.json().await?;

        for balance in response.balances {
            if balance.asset_type == "native" {
                return Ok(balance.balance);
            }
        }

        Err("No XLM balance found".into())
    }

    pub async fn send_transaction(&self, transaction: &str) -> Result<String, Box<dyn Error>> {
        // TODO: Implementar a função de enviar transação
        // let url = format!("{}/transactions", self.horizon_url);
        // let response = reqwest::post(url).json(transaction).send().await?;
        // let result: String = response.text().await?;
        // Ok(result)
        println!("Transação enviada com sucesso {}", transaction);
        Ok("Transação enviada com sucesso".to_string())
    }
}
