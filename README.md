# bulk-ether

ライブラリ更新した時
cargo build

動かしたい時
cargo run


windows用のバイナリ作成方法
(一回でいい)
rustup target add x86_64-pc-windows-gnu
(一回でいい)
brew install mingw-w64
cargo build --release --target x86_64-pc-windows-gnu
