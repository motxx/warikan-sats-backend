use anyhow::{Context, Result};
use gl_client::signer::Signer;
use std::path::Path;

use gl_client::bitcoin::Network;
use gl_client::scheduler::Scheduler;
use gl_client::tls::TlsConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let cert_path = Path::new("./gl-certs/client.crt");
    let device_cert = std::fs::read(cert_path).with_context(|| "Failed to read client.crt")?;

    let key_path = Path::new("./gl-certs/client-key.pem");
    let device_key = std::fs::read(key_path).with_context(|| "Failed to read client_key.pem")?;

    let tls = TlsConfig::new()
        .with_context(|| "Failed to create TLS configuration")?
        .identity(device_cert, device_key);

    let mnemonic_path = Path::new("./gl-certs/signer_mnemonic.txt");
    let mnemonic_words =
        std::fs::read(mnemonic_path).with_context(|| "Failed to read mnemonic_.txt")?;
    let signer = Signer::new(mnemonic_words, Network::Testnet, tls.clone())
        .with_context(|| "Failed to create signer")?;
    let node_id = signer.node_id();

    let scheduler = Scheduler::new(node_id, Network::Testnet).await?;
    let node: gl_client::node::ClnClient = scheduler.schedule(tls).await?;

    Ok(())
}
