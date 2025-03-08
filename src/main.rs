use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use clap::Parser;

use instant_acme::{
    Account,NewAccount,ExternalAccountKey
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Options::parse();

    // Create a new account. This will generate a fresh ECDSA key for you.

    let eab: Option<ExternalAccountKey> = match (opts.hmac_key_id, opts.hmac_key) {
        (Some(hmac_key_id), Some(hmac_key)) => {
            let hmac_key_decoded = BASE64_URL_SAFE_NO_PAD.decode(hmac_key).unwrap();
            Some(ExternalAccountKey::new(hmac_key_id, &hmac_key_decoded))
        },
        (_,_) => None
    };

    println!("EAB is set: {}", eab.is_some());

    let contact: &[&str] = match &opts.email {
        Some(email) => &[email],
        None => &[]
    };

    let (_account, credentials) = Account::create(
        &NewAccount {
            contact: contact,
            terms_of_service_agreed: true,
            only_return_existing: false,
        },
        &opts.server,
        eab.as_ref(),
    )
    .await?;

    println!(
        "account credentials:\n\n{}",
        serde_json::to_string_pretty(&credentials).unwrap()
    );
Ok(())
}

#[derive(Parser)]
struct Options {
    #[clap(long)]
    hmac_key_id: Option<String>,
    #[clap(long)]
    hmac_key: Option<String>,
    #[clap(long)]
    email: Option<String>,
    #[clap(long)]
    server: String,
}