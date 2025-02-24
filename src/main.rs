use dotenvy::dotenv;
use stellar_wallet::{Keys, Stellar};
// use stellar_wallet::Keys;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let stellar: Stellar = Stellar::new(&std::env::var("HORIZON_URL").unwrap());

    let balance = stellar
        .get_balance(&std::env::var("ACCOUNT_ID").unwrap())
        .await;

    println!("Balance: {:?}", balance);

    // let res = stellar
    //     .transfer_xlm(
    //         "SDLS7LO7QQ6QIFCBQCXZPGY6LYTA5KJTEECCQBKMZCDKUCSG63SHPCWC",
    //         "GDUWKWYRFET2CEKGVISMDJISRENQ7XCSOKDIX74NC7FFGXNAN6DTNMPA",
    //         "1",
    //     )
    //     .await;

    // println!("{:?}", res);
    // let public_key_from_private = Keys::get_public_key_from_private(
    //     &"SDLS7LO7QQ6QIFCBQCXZPGY6LYTA5KJTEECCQBKMZCDKUCSG63SHPCWC",
    // );
    // println!("Public Key: {:?}", public_key_from_private);

    // println!("Result: {:?}", result);

    match Keys::generate_stellar_keys() {
        Ok((public_key, secret_key)) => {
            println!("Public Key: {}", public_key);
            println!("Private Key: {}", secret_key);

            // let public_key_from_private = Keys::get_public_key_from_private(
            //     &"SDLS7LO7QQ6QIFCBQCXZPGY6LYTA5KJTEECCQBKMZCDKUCSG63SHPCWC",
            // );
            // println!("Public Key from Private: {:?}", public_key_from_private);
        }
        Err(e) => {
            eprintln!("Failed to generate keys: {}", e);
        }
    }
}
// Public Key: GBNQ5XKQAS6NK4DOYGOWCFJ6FRCJJXFEWKVMW7ILC3CNPTKNYU46RH2K
// Private Key: SCJBWCJBSCY6HTCZKW7RADJXC4IWU4JXNIYRAZ6WTQL5HUQU46QVZ3VC
