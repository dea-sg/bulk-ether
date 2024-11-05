pub mod provider;
pub mod wallet;

use ethers::prelude::*;
use std::sync::Arc;

/// クライアントを作成する関数
pub fn create_client(
    provider: Provider<Http>,
    wallet: LocalWallet,
    chain_id: u64, // チェーンIDを引数として追加
) -> Arc<SignerMiddleware<Provider<Http>, LocalWallet>> {
    let wallet = wallet.with_chain_id(chain_id);
    Arc::new(SignerMiddleware::new(provider, wallet))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::{Http, Provider};
    use ethers::signers::LocalWallet;
    // use std::sync::Arc;

    #[test]
    fn test_create_client() {
        // モックのプロバイダーを作成
        let provider = Provider::<Http>::try_from("https://localhost").unwrap();

        // モックのウォレットを作成
        let wallet: LocalWallet =
            "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .parse()
                .unwrap();

        // チェーンIDを指定してクライアントを作成
        let chain_id = 1u64; // Ethereum MainnetのチェーンID
        let client = create_client(provider, wallet, chain_id);

        // チェーンIDが正しく設定されているか確認
        assert_eq!(client.signer().chain_id(), chain_id);
    }
}
