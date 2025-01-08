# bulk-ether
## 概要
指定したアドレスに基軸通貨を配布するためのツール。
## 使い方
### 準備
1. 適当なフォルダを作成する
2. [releasesページ](https://github.com/dea-sg/bulk-ether/releases)から最新の環境に応じたbulk-ether.(exe.)zipをダウンロードし、解凍する
3. アドレスを記載したcsvファイルを作成する、名前はtarget.csvとする

```target.csv
0x000000........1
0x000000........2
0x000000........3
```
4. 1に作成したフォルダに2で解凍したファイルと3で作成したcsvファイルを配置する
### 実行
1. bulk-ether(bulk-ether.exe)をダブルクリックする
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

// macはこっち
// これでtarget/release/bulk-ether が生成される
cargo build --release

```
