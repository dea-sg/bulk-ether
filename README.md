# bulk-ether
## 概要
指定したアドレスに基軸通貨を配布するためのツール。
## 使い方(Windowsの場合)
### 準備
1. 適当なフォルダを作成する
2. releasesページから最新のbulk-ether.zipをダウンロードし、解凍する
3. アドレスを記載したcsvファイルを作成する、名前はtarget.csvとする
4. 1に作成したフォルダに2で解凍したファイルと3で作成したcsvファイルを配置する
### 実行
1. bulk-ether.exeをダブルクリックする
2. 「Please set Alchemy API key:」と表示されるので、AlchemyのAPIキーを入力する
3. 「Please set Ethereum private key: 」と表示されるので、配布元アドレスの秘密鍵を入力する
4. 「Please set value (default is 0.001):」と表示されるので、配布するETHの量を入力する。何も入力しない場合は0.001ETHが配布される
5. 「Please select the network:」と表示されるので、メインネットワークかテストネットワークを選択する

////////////////////////////////////////////////////////////////////////
## 以下、開発者向けメモ
### 開発中に使うコマンド
```
// とりあえず動かしたい時
cargo run

// ライブラリを更新した後
cargo build

// lint
cargo clippy

// format
cargo fmt

```
## Windows版のビルド
```
// 一回だけ準びのために実行する
rustup target add x86_64-pc-windows-gnu
brew install mingw-w64

// ビルド
// これでtarget/x86_64-pc-windows-gnu/release/bulk-ether.exeが生成される
cargo build --release --target x86_64-pc-windows-gnu

```
