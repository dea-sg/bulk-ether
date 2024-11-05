mod eth_client;
mod utils;
use ethers::prelude::*;
use ethers::utils::parse_units;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin(); // 標準入力を取得
    let mut stdin_reader = stdin.lock(); // `BufRead`を実装するようにロック
                                         // APIキーと秘密鍵を入力させる
    let api_key = utils::get_api_key(&mut stdin_reader)?;
    println!("API key: {}", api_key);

    let private_key = utils::get_private_key(&mut stdin_reader)?;
    println!("Private key set successfully.");

    // 配布したいetherの量を入力させる
    let value = utils::get_value(&mut stdin_reader)?; // エラーは自動的に伝播される
    println!("Value to send: {}", value);

    // switch_network_id 関数を使用して、チェーンIDを選択
    let chain_id = utils::switch_network_id(&mut stdin_reader)?;
    println!("Selected chain ID: {}", chain_id);

    // プロバイダー、ウォレット、クライアントを作成
    let provider = eth_client::provider::create_provider(&api_key, chain_id)?;
    let wallet = eth_client::wallet::create_wallet(&private_key)?;
    // チェーンIDを指定してクライアントを作成
    let client = eth_client::create_client(provider, wallet, chain_id);

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
        let tx = TransactionRequest::new().to(address).value(amount_in_wei);

        // トランザクションを送信
        match client.send_transaction(tx, None).await {
            Ok(pending_tx) => {
                let tx_hash = pending_tx.tx_hash();
                println!(
                    "Transaction sent to {} with tx hash: {:?}",
                    address, tx_hash
                );
            }
            Err(err) => {
                println!("Failed to send transaction to {}: {:?}", address, err);
            }
        }
    }

    Ok(())
}
