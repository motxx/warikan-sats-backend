mod services;

use anyhow::Result;
use dotenvy::dotenv;
use services::breez_client::{BreezClient, BreezConfig, InvoiceRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file not found");
    let config = BreezConfig {
        api_key: env::var("BREEZ_API_KEY")?,
        working_dir: env::var("BREEZ_WORKING_DIR")?,
        mnemonic: env::var("TEST_ONLY_ONCHAIN_MNEMONIC")?,
        key_path: env::var("PARTNER_DEVICE_KEY_PATH")?,
        cert_path: env::var("PARTNER_DEVICE_CERT_PATH")?,
    };

    let client = BreezClient::new(config).await?;
    let invoice = client
        .create_invoice(InvoiceRequest {
            amount_sat: 3000,
            description: "Bitcoin Transfer".into(),
        })
        .await?;

    println!("Invoice: {:?}", invoice.bolt11);
    Ok(())
}
