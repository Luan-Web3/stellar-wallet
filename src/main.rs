use dotenvy::dotenv;
use stellar_wallet::{Keys, Stellar};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _stellar: Stellar = Stellar::new(&std::env::var("HORIZON_URL").unwrap());

    // let balance = stellar
    //     .get_balance(&std::env::var("ACCOUNT_ID").unwrap())
    //     .await;

    // println!("Balance: {:?}", balance);

    let m: String = Keys::generate_mnemonic();
    let keys: Keys = Keys::from_mnemonic(&m);
    println!("Keys: {:?}", keys);
}
