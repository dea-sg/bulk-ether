pub mod provider;
pub mod wallet;

use ethers::prelude::*;
use std::sync::Arc;

pub fn create_client(
    provider: Provider<Http>,
    wallet: LocalWallet,
) -> Arc<SignerMiddleware<Provider<Http>, LocalWallet>> {
    let wallet = wallet.with_chain_id(11155111u64);  // SepoliaのチェーンID
    Arc::new(SignerMiddleware::new(provider, wallet))
}