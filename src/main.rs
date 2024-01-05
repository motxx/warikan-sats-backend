use std::env;

use anyhow::Result;
use bip39::{Language, Mnemonic};
use breez_sdk_core::{
    BreezEvent, BreezServices, EnvironmentType, EventListener, GreenlightNodeConfig,
};
use dotenvy::dotenv;

struct AppEventListener;

impl EventListener for AppEventListener {
    fn on_event(&self, e: BreezEvent) {
        println!("Received event: {:?}", e);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file not found");
    let api_key = env::var("BREEZ_API_KEY").expect("BREEZ_API_KEY not found in .env file");
    let invite_code =
        Some(env::var("BREEZ_INVITE_CODE").expect("BREEZ_INVITE_CODE not found in .env file"));

    let mnemonic = Mnemonic::generate_in(Language::English, 12)?;
    let seed = mnemonic.to_seed("");

    // Create the default config
    let mut config = BreezServices::default_config(
        EnvironmentType::Production,
        api_key,
        breez_sdk_core::NodeConfig::Greenlight {
            config: GreenlightNodeConfig {
                partner_credentials: None,
                invite_code,
            },
        },
    );

    // Customize the config object according to your needs
    config.working_dir = "path to an existing directory".into();

    // Connect to the Breez SDK make it ready for use
    let sdk = BreezServices::connect(config, seed.to_vec(), Box::new(AppEventListener {})).await?;
    Ok(())
}
