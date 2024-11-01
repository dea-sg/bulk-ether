mod eth_client;
mod utils;

use ethers::prelude::*;
use ethers::utils::parse_units;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // APIキーと秘密鍵を入力させる
    let api_key = utils::get_api_key()?;
    let private_key = utils::get_private_key()?;

    // トランザクションの値を入力させる
    let value = utils::get_value()?;
    println!("Transaction value set to: {}", value);

    // プロバイダー、ウォレット、クライアントを作成
    let provider = eth_client::provider::create_provider(&api_key)?;
    let wallet = eth_client::wallet::create_wallet(&private_key)?;
    let client = eth_client::create_client(provider.clone(), wallet.clone());

    // CSVファイルからアドレスのリストを読み込む
    let file_path = "target.csv";
    let addresses = utils::read_addresses_from_csv(file_path)?;
    println!("Loaded addresses from CSV: {:?}", addresses);

    // 各アドレスに送金
    for address_str in addresses {
        // 送金先アドレスをパース
        let address: Address = address_str.parse()?;
        
        // 送金する値をweiに変換
        let amount_in_wei = parse_units(value, "ether")?;

        // トランザクションの作成
        let tx = TransactionRequest::new()
            .to(address)
            .value(amount_in_wei);

        // トランザクションを送信
        match client.send_transaction(tx, None).await {
            Ok(pending_tx) => {
                let tx_hash = pending_tx.tx_hash();
                println!("Transaction sent to {} with tx hash: {:?}", address, tx_hash);
            }
            Err(err) => {
                println!("Failed to send transaction to {}: {:?}", address, err);
            }
        }
    }

    // 現在のブロック番号を取得
    let block_number = provider.get_block_number().await?;
    println!("Current block number: {}", block_number);

    Ok(())
}
