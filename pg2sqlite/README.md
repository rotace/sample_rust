# PG2SQLITE
PostgreSQLのデータベースをSQLite3ファイルにエクスポートするツール

## 設計指針
* CUI処理のため、同期Rustとして設計する。
* バイナリファイル直下のsetting.jsonを読み込む。
* setting.jsonにて、データベースURIを指定できる。
* setting.jsonにて、任意で特定のテーブルを指定してレコード削除可否などのオプションを実装できる拡張性を有する。
* 指定したデータベース内の全てのテーブルをDataFrameとして取り込み、SQLite3に出力する。
* デーベースURIにアクセスする際は、postgresクレートを使用する。
* DataFrameにロードする際は、polarsクレートを使用する。
* sqlite3に書き込む際は、rusqliteクレートを使用する。

## TODO
* postgresqlのインストールとサンプルデータ作成
