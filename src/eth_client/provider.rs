use ethers::prelude::*;
use std::convert::TryFrom;

/// Alchemyプロバイダーの作成
pub fn create_provider(api_key: &str) -> Result<Provider<Http>, Box<dyn std::error::Error>> {
    let provider_url = format!("https://eth-sepolia.g.alchemy.com/v2/{}", api_key);
    let provider = Provider::<Http>::try_from(provider_url)?;
    Ok(provider)
}