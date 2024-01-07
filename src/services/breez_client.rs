use std::sync::Arc;
use std::{path::Path, str::FromStr};

use anyhow::Result;
use bip39::Mnemonic;
use breez_sdk_core::{
    BreezEvent, BreezServices, EnvironmentType, EventListener, GreenlightCredentials,
    GreenlightNodeConfig, LNInvoice, ReceivePaymentRequest,
};
use rand::Rng;
struct AppEventListener;

impl EventListener for AppEventListener {
    fn on_event(&self, e: BreezEvent) {
        println!("Received event: {:?}", e);
    }
}

pub struct BreezConfig {
    pub api_key: String,
    pub working_dir: String,
    pub mnemonic: String,
    pub key_path: String,
    pub cert_path: String,
}

pub struct BreezClient {
    pub sdk: Arc<BreezServices>,
}

pub struct InvoiceRequest {
    pub amount_sat: u64,
    pub description: String,
}

pub struct Invoice {
    pub bolt11: String,
}

impl BreezClient {
    pub async fn new(config: BreezConfig) -> Result<Self> {
        let key_path = Path::new(&config.key_path);
        let device_key = std::fs::read(key_path)?;
        let cert_path = Path::new(&config.cert_path);
        let device_cert = std::fs::read(cert_path)?;

        let mut breez_config = BreezServices::default_config(
            EnvironmentType::Production,
            config.api_key,
            breez_sdk_core::NodeConfig::Greenlight {
                config: GreenlightNodeConfig {
                    partner_credentials: Some(GreenlightCredentials {
                        device_key,
                        device_cert,
                    }),
                    invite_code: None,
                },
            },
        );
        breez_config.working_dir = config.working_dir.into();

        let mnemonic = Mnemonic::from_str(&config.mnemonic)?;
        let seed = mnemonic.to_seed("");
        let sdk =
            BreezServices::connect(breez_config, seed.to_vec(), Box::new(AppEventListener {}))
                .await?;
        Ok(BreezClient { sdk })
    }

    pub async fn create_invoice(&self, req: InvoiceRequest) -> Result<Invoice> {
        let preimage = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let expiry_seconds = 120;
        let response = self
            .sdk
            .receive_payment(ReceivePaymentRequest {
                amount_msat: req.amount_sat * 1000,
                description: req.description,
                preimage: Some(preimage),
                opening_fee_params: None,
                use_description_hash: Some(false),
                expiry: Some(expiry_seconds),
                cltv: None,
            })
            .await?;

        let ln_invoice: LNInvoice = response.ln_invoice;
        Ok(Invoice {
            bolt11: ln_invoice.bolt11,
        })
    }
}
