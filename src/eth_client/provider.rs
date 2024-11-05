use ethers::prelude::*;
use std::convert::TryFrom;

/// Alchemyプロバイダーの作成
pub fn create_provider(
    api_key: &str,
    chain_id: u64,
) -> Result<Provider<Http>, Box<dyn std::error::Error>> {
    // チェーンIDに応じてプロバイダーURLを切り替える
    let provider_url: String = match chain_id {
        1 => format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key),
        11155111 => format!("https://eth-sepolia.g.alchemy.com/v2/{}", api_key),
        _ => return Err("Unsupported chain ID".into()),
    };
    // プロバイダーを作成
    let provider = Provider::<Http>::try_from(provider_url)?;
    Ok(provider)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider_mainnet() {
        // テスト用のダミーAPIキーを使用
        let api_key = "dummy_api_key";
        let chain_id = 1; // Ethereum MainnetのチェーンID

        // create_provider関数をテスト
        let provider_result = create_provider(api_key, chain_id);
        assert!(provider_result.is_ok()); // プロバイダーが正しく作成されることを確認

        // URLの正しさを確認するための追加テスト
        if let Ok(provider) = provider_result {
            let provider_url = provider.url().to_string();
            assert_eq!(
                provider_url,
                "https://eth-mainnet.g.alchemy.com/v2/dummy_api_key"
            );
        }
    }

    #[test]
    fn test_create_provider_sepolia() {
        // テスト用のダミーAPIキーを使用
        let api_key = "dummy_api_key";
        let chain_id = 11155111; // SepoliaのチェーンID

        // create_provider関数をテスト
        let provider_result = create_provider(api_key, chain_id);
        assert!(provider_result.is_ok()); // プロバイダーが正しく作成されることを確認

        // URLの正しさを確認するための追加テスト
        if let Ok(provider) = provider_result {
            let provider_url = provider.url().to_string();
            assert_eq!(
                provider_url,
                "https://eth-sepolia.g.alchemy.com/v2/dummy_api_key"
            );
        }
    }

    #[test]
    fn test_create_provider_unsupported_chain() {
        // テスト用のダミーAPIキーを使用
        let api_key = "dummy_api_key";
        let chain_id = 9999; // サポートされていないチェーンID

        // create_provider関数をテスト
        let provider_result = create_provider(api_key, chain_id);
        assert!(provider_result.is_err()); // サポートされていないチェーンIDではエラーが返されることを確認
    }
}
