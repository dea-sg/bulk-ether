use csv::ReaderBuilder;
use ethers::signers::LocalWallet;
use ethers::types::Address;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{self, Write};
use std::str::FromStr;

/// APIキーの入力を取得
pub fn get_api_key<R: BufRead>(reader: &mut R) -> Result<String, Box<dyn std::error::Error>> {
    print!("Please set Alchemy API key: ");
    io::stdout().flush()?;

    let mut api_key = String::new();
    reader.read_line(&mut api_key)?;
    // api_keyが0文字の場合はエラーを返す
    if api_key.trim().is_empty() {
        return Err("API key cannot be empty".into());
    }
    Ok(api_key.trim().to_string())
}

/// 秘密鍵の入力を取得
pub fn get_private_key<R: BufRead>(reader: &mut R) -> Result<String, Box<dyn std::error::Error>> {
    print!("Please set Ethereum private key: ");
    io::stdout().flush()?;

    let mut private_key = String::new();
    reader.read_line(&mut private_key)?;
    let private_key = private_key.trim();

    // ethers::signers::LocalWallet を使用してパースを試みる
    match private_key.parse::<LocalWallet>() {
        Ok(_) => Ok(private_key.to_string()),
        Err(_) => Err("Invalid private key. Please enter a valid private key.".into()),
    }
}
/// CSVファイルからEVMアドレスを読み取る関数
pub fn read_addresses_from_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false) // CSVにヘッダーがない場合はfalseに設定
        .from_reader(BufReader::new(file));

    let mut addresses = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let address = record
            .get(0)
            .ok_or("Failed to get address")?
            .trim()
            .to_string();
        // addressが妥当なEVMアドレスであることを確認
        if Address::from_str(&address).is_err() {
            return Err("Invalid address found in CSV".into());
        }
        addresses.push(address);
    }

    Ok(addresses)
}

/// 配布したいetherの量を取得
pub fn get_value<R: BufRead>(reader: &mut R) -> Result<f64, Box<dyn std::error::Error>> {
    print!("Please set value (default is 0.001): ");
    io::stdout().flush()?;

    let mut value_str = String::new();
    reader.read_line(&mut value_str)?;
    let value_str = value_str.trim();

    if value_str.is_empty() {
        return Ok(0.001);
    }

    // 入力がある場合は数値としてパースし、エラーチェックを行う
    match value_str.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Invalid value. Please enter a valid decimal number.".into()),
    }
}

/// 接続したいチェーンを選択
pub fn switch_network_id<R: BufRead>(reader: &mut R) -> Result<u64, Box<dyn std::error::Error>> {
    println!("Please select the network:");
    println!("1: Ethereum Mainnet");
    println!("2: Sepolia Testnet");
    print!("Enter your choice (1 or 2): ");
    io::stdout().flush()?;
    let mut network_choice = String::new();
    reader.read_line(&mut network_choice)?;
    let network_choice = network_choice.trim();

    match network_choice {
        "1" => Ok(1),
        "2" => Ok(11155111),
        _ => Err("Invalid network choice. Please enter 1 or 2.".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_get_private_key_valid() {
        // 有効な秘密鍵を標準入力として与える
        let input = b"0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\n";
        let mut cursor = Cursor::new(input);

        // get_private_key 関数をテストし、秘密鍵が正常に取得されることを確認
        let result = get_private_key(&mut cursor).unwrap();
        assert_eq!(
            result,
            "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        );
    }

    #[test]
    fn test_get_private_key_invalid() {
        // 無効な秘密鍵を標準入力として与える
        let input = b"invalid_private_key\n";
        let mut cursor = Cursor::new(input);

        // 無効な秘密鍵の入力に対してエラーが返ることを確認
        let result = get_private_key(&mut cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_api_key() {
        // 有効なAPIキー "sample_api_key" を標準入力として与える
        let input = b"sample_api_key\n";
        let mut cursor = Cursor::new(input);

        // get_api_key 関数をテスト
        let result = get_api_key(&mut cursor).unwrap();
        assert_eq!(result, "sample_api_key");
    }

    #[test]
    fn test_get_api_key_empty() {
        // 空のAPIキー入力 (Enterキーだけ) を標準入力として与える
        let input = b"\n";
        let mut cursor = Cursor::new(input);

        // get_api_key 関数をテストし、空の入力に対してエラーが返ることを確認
        let result = get_api_key(&mut cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_value_default() {
        // 空の入力 (Enterキー相当) を標準入力として与える
        let input = b"\n";
        let mut cursor = Cursor::new(input);

        // get_value 関数をテスト
        let value = get_value(&mut cursor).unwrap();
        assert_eq!(value, 0.001);
    }

    #[test]
    fn test_get_value_specific() {
        // "0.005" の入力を標準入力として与える
        let input = b"0.005\n";
        let mut cursor = Cursor::new(input);

        // get_value 関数をテスト
        let value = get_value(&mut cursor).unwrap();
        assert_eq!(value, 0.005);
    }

    #[test]
    fn test_get_value_invalid() {
        // 無効な入力 "invalid" を標準入力として与える
        let input = b"invalid\n";
        let mut cursor = Cursor::new(input);

        // get_value 関数をテストし、エラーが返ることを確認
        let result = get_value(&mut cursor);
        assert!(result.is_err());
    }
}
