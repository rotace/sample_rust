use std::io::BufRead;

use rusqlite::Connection;

#[derive(Debug, serde::Deserialize)]
struct WeatherRecord {
    #[serde(rename = "年月日時")]
    timestamp: String,
    #[serde(rename = "気温(℃)")]
    temperature: f64,
    #[serde(rename = "相対湿度(％)")]
    humidity: f64,
    #[serde(rename = "蒸気圧(hPa)")]
    vapor_pressure: f64,
    #[serde(rename = "露点温度(℃)")]
    dew_point: f64,
}

/// 気温データを読み込む
pub fn load_weather_data(conn: &mut Connection, file: &str) -> anyhow::Result<()> {
    println!("Loading weather data from file: {}", file);

    // データベースにテーブル（気温、相対湿度、蒸気圧、露点温度）を作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS weather (
            timestamp TIMESTAMP PRIMARY KEY,
            temperature REAL NOT NULL,
            humidity REAL NOT NULL,
            vapor_pressure REAL NOT NULL,
            dew_point REAL NOT NULL
        )",
        [],
    )?;

    // データベースにビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '月別平均気象量' AS
        WITH monthly_avg AS (
            SELECT
                strftime('%Y-%m', timestamp) AS month,
                AVG(temperature) AS avg_temperature,
                AVG(humidity) AS avg_humidity,
                AVG(vapor_pressure) AS avg_vapor_pressure,
                AVG(dew_point) AS avg_dew_point
            FROM weather
            GROUP BY month
        )
        SELECT
            month AS '月',
            avg_temperature AS '平均気温[℃]',
            avg_humidity AS '平均相対湿度[％]',
            avg_vapor_pressure AS '平均蒸気圧[hPa]',
            avg_dew_point AS '平均露点温度[℃]'
        FROM monthly_avg
        ORDER BY month",
        [],
    )?;

    // CSVファイルを開く
    let file = std::fs::File::open(file)?;
    // Shift_JISをUTF-8に変換しながら読み込む
    let transcoded = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::SHIFT_JIS))
        .build(file);
    // BufReaderでラップして、先頭３行をスキップ
    let mut buffered = std::io::BufReader::new(transcoded);
    let mut dummy = String::new();
    for _ in 0..3 {
        buffered.read_line(&mut dummy)?;
    }
    // CSVリーダーを作成
    let mut reader = csv::Reader::from_reader(buffered);
    // トランザクション開始
    let mut counter = 0;
    let tx = conn.transaction()?;
    for result in reader.deserialize::<WeatherRecord>() {
        counter += 1;
        if counter % 1000 == 0 {
            println!("Processed {} records...", counter);
        }
        match result {
            Err(e) => {
                eprintln!("Error parsing record: {}", e);
                continue;
            }
            Ok(record) => {
                let timestamp =
                    chrono::NaiveDateTime::parse_from_str(&record.timestamp, "%Y/%m/%d %H:%M")?;
                tx.execute(
                    "INSERT OR REPLACE INTO weather (timestamp, temperature, humidity, vapor_pressure, dew_point) VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![
                        timestamp.to_string(),
                        record.temperature,
                        record.humidity,
                        record.vapor_pressure,
                        record.dew_point
                    ],
                )?;
            }
        }
    }
    tx.commit()?;
    println!("Finished loading weather data. Total records: {}", counter);
    Ok(())
}
