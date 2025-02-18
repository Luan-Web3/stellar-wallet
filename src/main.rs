use dotenvy::dotenv;
use stellar_wallet::Stellar;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let stellar: Stellar = Stellar::new(&std::env::var("HORIZON_URL").unwrap());

    let balance = stellar
        .get_balance(&std::env::var("ACCOUNT_ID").unwrap())
        .await;

    println!("Balance: {:?}", balance);
}
