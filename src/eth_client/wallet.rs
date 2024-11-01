use ethers::prelude::*;

/// ウォレットの作成
pub fn create_wallet(private_key: &str) -> Result<LocalWallet, Box<dyn std::error::Error>> {
    let wallet: LocalWallet = private_key.parse()?;
    Ok(wallet)
}