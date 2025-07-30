use polars::prelude::*;
use postgres::{Client, NoTls};
use rusqlite::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    pg_uri: String,
    sqlite_path: String,
    // Optionally: tables: Option<Vec<String>>,
    // Optionally: delete_options: Option<serde_json::Value>,
}

/// PostgreSQLからデータを読み込み、DataFrameに変換する
fn read_database_uri(uri: &str, query: &str) -> anyhow::Result<DataFrame> {
    let mut client = Client::connect(&uri, postgres::NoTls)?;
    let mut columns: Vec<Vec<String>> = vec![]; // カラムごとに格納する（型は一例）
    let mut column_names: Vec<String> = vec![];

    for row in client.query(query, &[])? {
        for (i, col) in row.columns().iter().enumerate() {
            if columns.len() <= i {
                columns.push(vec![]);
                column_names.push(col.name().to_string());
            }
            let value: String = row.get(i);
            columns[i].push(value);
        }
    }

    let series_list: Vec<Column> = column_names
        .into_iter()
        .zip(columns)
        .map(|(name, data)| Column::new(name.into(), data))
        .collect();

    Ok(DataFrame::new(series_list)?)
}

/// DataFrameをSQLiteに書き込む
fn write_df_to_sqlite(conn: &Connection, df: &DataFrame, table_name: &str) -> anyhow::Result<()> {
    // スキーマの自動生成（カラム名と型に基づく）
    let schema: Vec<(String, &DataType)> = df
        .get_columns()
        .iter()
        .map(|col| (col.name().to_string(), col.dtype()))
        .collect();

    let column_defs: Vec<String> = schema
        .iter()
        .map(|(name, dtype)| {
            let sql_type = match dtype {
                DataType::Int64 | DataType::Int32 => "INTEGER",
                DataType::Float64 | DataType::Float32 => "REAL",
                DataType::String => "TEXT",
                DataType::Boolean => "BOOLEAN",
                _ => "BLOB", // Fallback
            };
            format!("{name} {sql_type}")
        })
        .collect();

    let create_stmt = format!(
        "CREATE TABLE IF NOT EXISTS {table_name} ({})",
        column_defs.join(", ")
    );
    conn.execute(&create_stmt, [])?;

    // 行ごとに挿入
    for row in 0..df.height() {
        let values: Vec<String> = df
            .get_columns()
            .iter()
            .map(|col| -> anyhow::Result<String> {
                let ret = match col.get(row)? {
                    AnyValue::Int64(v) => v.to_string(),
                    AnyValue::Float64(v) => v.to_string(),
                    AnyValue::String(v) => format!("'{}'", v.replace("'", "''")),
                    AnyValue::Boolean(v) => (v as i32).to_string(),
                    _ => "NULL".to_string(),
                };
                Ok(ret)
            })
            .collect::<anyhow::Result<Vec<String>>>()?;

        let insert_stmt = format!(
            "INSERT INTO {table_name} ({}) VALUES ({})",
            schema
                .iter()
                .map(|(n, _)| n.clone())
                .collect::<Vec<_>>()
                .join(", "),
            values.join(", ")
        );
        conn.execute(&insert_stmt, [])?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // setting.jsonの読み込み
    let config: Settings = {
        let data = std::fs::read_to_string("setting.json")?;
        serde_json::from_str(&data)?
    };

    // PostgreSQLへ接続
    let mut pg_client = Client::connect(&config.pg_uri, NoTls)?;

    // SQLiteへ接続
    let sqlite_conn = Connection::open(&config.sqlite_path)?;

    // テーブル一覧取得
    let tables = pg_client.query(
        "SELECT tablename FROM pg_tables WHERE schemaname = 'public'",
        &[],
    )?;

    for row in tables {
        let table_name: &str = row.get(0);

        // テーブル名を取得
        let query = format!("SELECT * FROM {}", table_name);

        // DataFrame化
        let df = read_database_uri(&config.pg_uri, &query)?;

        // SQLiteに書き込み
        write_df_to_sqlite(&sqlite_conn, &df, table_name)?;
        println!("Exported table: {}", table_name);
    }

    println!("Export completed.");
    Ok(())
}
