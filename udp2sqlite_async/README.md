# UDP2SQLITE(非同期Rust版)
UDPで受信したSQL又はバイナリデータをSQLite3ファイルにエクスポートするツール

## Design
* 常駐型の非同期Rustとして設計する。
* bin/フォルダにserver.rsとclient.rsを用意する。
* server.rsはUDPポート:3000でSQL文を待受する。
* client.rsはUDPポート:3000にSQL文を送信する。
* server.rsはUDPポート:4000でバイナリデータを待受する。
* client.rsはUDPポート:4000にバイナリデータを送信する。
* binrwクレートを使用してバイナリデータを構造体に変換する。
* sea-ormクレートを使用して構造体をsqlite3に保存する。
* sea-ormクレートを使用してSQL文をsqlite3で実行する。
