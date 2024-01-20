use anyhow::{Context, Ok, Result};
use bip39::{Language, Mnemonic};
use gl_client::bitcoin::hashes::hex::ToHex;
use gl_client::bitcoin::Network;
use gl_client::pb::cln;
use gl_client::scheduler::Scheduler;
use gl_client::signer::Signer;
use gl_client::tls::TlsConfig;
use std::path::Path;

fn get_tls_config() -> Result<TlsConfig> {
    let cert_path = Path::new("./gl-certs/client.crt");
    let device_cert = std::fs::read(cert_path).with_context(|| "Failed to read client.crt")?;

    let key_path = Path::new("./gl-certs/client-key.pem");
    let device_key = std::fs::read(key_path).with_context(|| "Failed to read client_key.pem")?;

    let tls = TlsConfig::new()
        .with_context(|| "Failed to create TLS configuration")?
        .identity(device_cert, device_key);

    Ok(tls)
}

fn get_mnemonic() -> Result<Vec<u8>> {
    let mnemonic_path = Path::new("./signers/mnemonic.txt");
    let secret = std::fs::read(mnemonic_path).unwrap_or_else(|_| {
        println!("Failed to read {}", mnemonic_path.display());
        println!("Generating new mnemonic to {}", mnemonic_path.display());
        let mut rng = rand::thread_rng();
        let m = Mnemonic::generate_in_with(&mut rng, Language::English, 24)
            .expect("Failed to generate mnemonic");
        // let phrase = m.word_iter().fold("".to_string(), |c, n| c + " " + n);
        let seed = &m.to_seed("")[0..32]; // Only need the first 32 bytes
        std::fs::write(mnemonic_path, seed[0..32].to_vec()).expect("Failed to write mnemonic.txt");
        seed[0..32].to_vec()
    });
    Ok(secret)
}

async fn test_payment(mut node: gl_client::node::ClnClient) -> Result<()> {
    let res = node.getinfo(cln::GetinfoRequest::default()).await;
    res.map(|r| println!("getinfo: {:?}", r))
        .map_err(|e| println!("getinfo error: {:?}", e))
        .ok();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let secret = get_mnemonic()?;
    let network = Network::Bitcoin;
    let tls = get_tls_config()?;
    let signer =
        Signer::new(secret, network, tls.clone()).with_context(|| "Failed to create signer")?;

    println!("Signer node id: {}", signer.node_id().to_hex());
    let scheduler = Scheduler::new(signer.node_id(), network).await?;
    if let Err(_) = scheduler.register(&signer, None).await {
        println!("Signer already registered");
    }
    let node: gl_client::node::ClnClient = scheduler.schedule(tls).await?;
    println!("Scheduler is working...");

    test_payment(node).await?;

    Ok(())
}
