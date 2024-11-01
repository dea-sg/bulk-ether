use std::io::{self, Write};
use rpassword;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;
use ethers::signers::LocalWallet;

/// APIキーの入力を取得
pub fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    print!("Please set Alchemy API key: ");
    io::stdout().flush()?; // 出力をフラッシュしてユーザーがプロンプトをすぐに見られるようにする
    let api_key = rpassword::read_password()?;
    Ok(api_key.trim().to_string())
}

/// 秘密鍵の入力を取得
pub fn get_private_key() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        print!("Please set Ethereum private key: ");
        io::stdout().flush()?;
        let private_key = rpassword::read_password()?;
        let private_key = private_key.trim();

        // ethers::signers::LocalWallet を使用してパースを試みる
        match private_key.parse::<LocalWallet>() {
            Ok(_) => return Ok(private_key.to_string()),
            Err(_) => println!("Invalid private key. Please enter a valid private key."),
        }
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
        let address = record.get(0).ok_or("Failed to get address")?.trim().to_string();
        addresses.push(address);
    }

    Ok(addresses)
}

/// トランザクションの値を取得
pub fn get_value() -> Result<f64, Box<dyn std::error::Error>> {
    print!("Please set value (default is 0.001): ");
    io::stdout().flush()?;

    let mut value_str = String::new();
    io::stdin().read_line(&mut value_str)?;
    let value_str = value_str.trim();

    if value_str.is_empty() {
        // 未入力の場合はデフォルト値を返す
        return Ok(0.001);
    }

    // 入力がある場合は数値としてパースし、エラーチェックを行う
    match value_str.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Invalid value. Please enter a valid decimal number.".into()),
    }
}