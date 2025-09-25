use rusqlite::{Connection, params};

#[derive(Debug, serde::Deserialize)]
struct PowerRecord {
    #[serde(rename = "ご使用日")]
    timestamp: String,

    #[serde(rename = "00:00-01:00")]
    usage00: f64,

    #[serde(rename = "01:00-02:00")]
    usage01: f64,

    #[serde(rename = "02:00-03:00")]
    usage02: f64,

    #[serde(rename = "03:00-04:00")]
    usage03: f64,

    #[serde(rename = "04:00-05:00")]
    usage04: f64,

    #[serde(rename = "05:00-06:00")]
    usage05: f64,

    #[serde(rename = "06:00-07:00")]
    usage06: f64,

    #[serde(rename = "07:00-08:00")]
    usage07: f64,

    #[serde(rename = "08:00-09:00")]
    usage08: f64,

    #[serde(rename = "09:00-10:00")]
    usage09: f64,

    #[serde(rename = "10:00-11:00")]
    usage10: f64,

    #[serde(rename = "11:00-12:00")]
    usage11: f64,

    #[serde(rename = "12:00-13:00")]
    usage12: f64,

    #[serde(rename = "13:00-14:00")]
    usage13: f64,

    #[serde(rename = "14:00-15:00")]
    usage14: f64,

    #[serde(rename = "15:00-16:00")]
    usage15: f64,

    #[serde(rename = "16:00-17:00")]
    usage16: f64,

    #[serde(rename = "17:00-18:00")]
    usage17: f64,

    #[serde(rename = "18:00-19:00")]
    usage18: f64,

    #[serde(rename = "19:00-20:00")]
    usage19: f64,

    #[serde(rename = "20:00-21:00")]
    usage20: f64,

    #[serde(rename = "21:00-22:00")]
    usage21: f64,

    #[serde(rename = "22:00-23:00")]
    usage22: f64,

    #[serde(rename = "23:00-24:00")]
    usage23: f64,
}

/// 日本語の日付形式をパースする関数
fn parse_japanese_date(date_str: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(date_str, "%Y年%m月%d日").ok()
}

/// 消費電力量データを読み込む
pub fn load_power_data(conn: &mut Connection, file: &str) -> anyhow::Result<()> {
    println!("Loading power data from file: {}", file);

    // データベースにテーブル作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS '時別消費電力量' (
            timestamp TIMESTAMP PRIMARY KEY,
            match_key TEXT GENERATED ALWAYS AS (STRFTIME('%m-%d %H:00', timestamp)) STORED,
            '消費電力量[kWh]' REAL
        )",
        [],
    )?;

    // データベースに月別消費電力量ビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '月別消費電力量' AS
        WITH monthly_totals AS (
            SELECT
                strftime('%Y-%m', timestamp) AS month,
                SUM('消費電力量[kWh]') AS total_power
            FROM '時別消費電力量'
            GROUP BY month
        )
        SELECT 
            month AS '年月',
            total_power AS '月間使用量[kWh]'
        FROM monthly_totals
        ORDER BY month",
        [],
    )?;

    // CSVファイルを開く
    let file = std::fs::File::open(file)?;
    // CSVリーダーを作成
    let mut reader = csv::Reader::from_reader(file);
    // トランザクション開始
    let mut counter = 0;
    let mut tx = conn.transaction()?;
    for result in reader.deserialize::<PowerRecord>() {
        counter += 1;
        if counter % 100 == 0 {
            tx.commit()?;
            tx = conn.transaction()?;
            println!("Processed {} records...", counter);
        }
        match result {
            Err(e) => {
                eprintln!("Error parsing record: {}", e);
                continue;
            }
            Ok(record) => {
                let date = parse_japanese_date(&record.timestamp).unwrap();
                let sql =
                    "INSERT INTO '時別消費電力量' (timestamp, '消費電力量[kWh]') VALUES (?1, ?2)";
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(0, 0, 0).unwrap().to_string(),
                        &record.usage00,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(1, 0, 0).unwrap().to_string(),
                        &record.usage01,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(2, 0, 0).unwrap().to_string(),
                        &record.usage02,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(3, 0, 0).unwrap().to_string(),
                        &record.usage03,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(4, 0, 0).unwrap().to_string(),
                        &record.usage04,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(5, 0, 0).unwrap().to_string(),
                        &record.usage05,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(6, 0, 0).unwrap().to_string(),
                        &record.usage06,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(7, 0, 0).unwrap().to_string(),
                        &record.usage07,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(8, 0, 0).unwrap().to_string(),
                        &record.usage08,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(9, 0, 0).unwrap().to_string(),
                        &record.usage09,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(10, 0, 0).unwrap().to_string(),
                        &record.usage10,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(11, 0, 0).unwrap().to_string(),
                        &record.usage11,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(12, 0, 0).unwrap().to_string(),
                        &record.usage12,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(13, 0, 0).unwrap().to_string(),
                        &record.usage13,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(14, 0, 0).unwrap().to_string(),
                        &record.usage14,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(15, 0, 0).unwrap().to_string(),
                        &record.usage15,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(16, 0, 0).unwrap().to_string(),
                        &record.usage16,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(17, 0, 0).unwrap().to_string(),
                        &record.usage17,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(18, 0, 0).unwrap().to_string(),
                        &record.usage18,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(19, 0, 0).unwrap().to_string(),
                        &record.usage19,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(20, 0, 0).unwrap().to_string(),
                        &record.usage20,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(21, 0, 0).unwrap().to_string(),
                        &record.usage21,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(22, 0, 0).unwrap().to_string(),
                        &record.usage22,
                    ],
                )?;
                tx.execute(
                    sql,
                    params![
                        date.and_hms_opt(23, 0, 0).unwrap().to_string(),
                        &record.usage23,
                    ],
                )?;
            }
        }
    }
    tx.commit()?;
    println!("Finished loading power data. Total records: {}", counter);
    Ok(())
}
