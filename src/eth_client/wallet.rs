use ethers::prelude::*;

/// ウォレットの作成
pub fn create_wallet(private_key: &str) -> Result<LocalWallet, Box<dyn std::error::Error>> {
    let wallet: LocalWallet = private_key.parse()?;
    Ok(wallet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet_valid_private_key() {
        // 有効な秘密鍵を使用（64桁の16進数）
        let private_key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

        // ウォレットの作成をテスト
        let wallet_result = create_wallet(private_key);
        assert!(wallet_result.is_ok()); // ウォレットが正しく作成されることを確認

        // 作成されたウォレットの確認
        if let Ok(wallet) = wallet_result {
            let expected_address = wallet.address();
            println!("Generated address: {:?}", expected_address);

            // アドレスが正しい形式で生成されているかを確認
            assert!(expected_address.as_bytes().len() == 20);
        }
    }

    #[test]
    fn test_create_wallet_invalid_private_key() {
        // 不正な秘密鍵を使用
        let invalid_private_key = "invalid_private_key";

        // ウォレットの作成をテスト
        let wallet_result = create_wallet(invalid_private_key);
        assert!(wallet_result.is_err()); // 不正な秘密鍵ではエラーが返されることを確認
    }
}
