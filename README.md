Stellar Wallet
===========================

![GitHub repo size](https://img.shields.io/github/repo-size/Luan-Web3/stellar-wallet?style=for-the-badge)
![GitHub language count](https://img.shields.io/github/languages/count/Luan-Web3/stellar-wallet?style=for-the-badge)
![GitHub forks](https://img.shields.io/github/forks/Luan-Web3/stellar-wallet?style=for-the-badge)

This project is a Rust library for interacting with the Stellar network, offering essential functionalities for developers looking to integrate XLM operations into their applications. The library provides:

- **Key pair generation** to securely create new Stellar accounts.
- **Balance inquiry** to check the XLM and other token balances on the active account.
- **Receiving tokens from the Friendbot** on test networks, facilitating development and testing at no cost.
- **XLM transfers** between accounts, with support for transaction creation and signing.

## Instalation

```
cargo add stellar_wallet
```

## Examples

```rust
use stellar_wallet::Keys;

fn main() {
    let (public_key, private_key) = Keys::generate_stellar_keys().unwrap();
    ...

    let sodium_key_pair = Keys::get_public_key_from_private("SDLS7LO7QQ...")?;
    ...
}
```

```rust
use stellar_wallet::Stellar;

#[tokio::main]
async fn main() {
    let stellar = Stellar::new("https://horizon-testnet.stellar.org");

    let _ = stellar.fund_account_with_friendbot("GCR4EZFL7K...").await;
    ...

    let _ = stellar.get_balance("GCR4EZFL7K...").await;
    ...

    let _ = stellar.transfer_xlm("SDLS7LO7QQ...", "GCR4EZFL7K..", "1").await;
    ...
}
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
