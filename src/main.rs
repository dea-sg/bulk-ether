mod eth_client;
mod utils;
use ethers::prelude::*;
use ethers::utils::parse_units;
use std::io;
use std::time::Duration;
use tokio::time::sleep;

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
                // トランザクションがブロックに取り込まれるまで待機
                match pending_tx.await {
                    Ok(receipt) => {
                        if let Some(receipt) = receipt {
                            println!(
                                "Transaction confirmed for {} with status: {:?}",
                                address, receipt.status
                            );
                        } else {
                            println!(
                                "Transaction was dropped or could not be confirmed: {:?}",
                                tx_hash
                            );
                        }
                    }
                    Err(err) => {
                        println!("Failed to confirm transaction for {}: {:?}", address, err);
                    }
                }
            }
            Err(err) => {
                println!("Failed to send transaction to {}: {:?}", address, err);
            }
        }

        // トランザクションの間に少し待機時間を追加
        sleep(Duration::from_secs(15)).await; // 15秒待機してから次のトランザクションを送信
    }

    Ok(())
}
