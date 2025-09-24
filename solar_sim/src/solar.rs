use chrono::NaiveDate;
use rusqlite::{Connection, params};

/// 日照量データを読み込む
pub fn load_solar_data(conn: &mut Connection, file: &str) -> anyhow::Result<()> {
    println!("Loading solar data from file: {}", file);

    // データベースにテーブルを作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS solar (
            timestamp TIMESTAMP PRIMARY KEY,
            solar_kwh REAL
        )",
        [],
    )?;

    // データベースにビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '月別平均日射量' AS
        WITH daily_totals AS (
            SELECT
                date(timestamp) AS day,
                SUM(solar_kwh) AS daily_totals_kwh
            FROM solar
            GROUP BY day
        ),
        monthly_averages AS (
            SELECT
                strftime('%m', day) AS month,
                AVG(daily_totals_kwh) AS monthly_avg_kwh
            FROM daily_totals
            GROUP BY month
        )
        SELECT
            month AS '月',
            monthly_avg_kwh AS '日射量[kWh/m2/日]'
        FROM monthly_averages
        ORDER BY month",
        [],
    )?;

    // CSVファイルを開く
    let file = std::fs::File::open(file)?;
    // ヘッダと可変列を無視してCSVリーダーを作成
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(file);
    // トランザクションを開始
    let mut counter = 0;
    let mut tx = conn.transaction()?;
    for result in reader.records().skip(1) {
        counter += 1;
        if counter % 100 == 0 {
            tx.commit()?;
            tx = conn.transaction()?;
            println!("Processed {} records...", counter);
        }
        let record = result?;
        let date = NaiveDate::from_ymd_opt(
            record[4].parse().unwrap(),
            record[2].parse().unwrap(),
            record[3].parse().unwrap(),
        )
        .unwrap();
        let sql = "INSERT INTO solar (timestamp, solar_kwh) VALUES (?1, ?2)";
        tx.execute(
            sql,
            params![
                date.and_hms_opt(0, 0, 0).unwrap().to_string(),
                record[5].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(1, 0, 0).unwrap().to_string(),
                record[6].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(2, 0, 0).unwrap().to_string(),
                record[7].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(3, 0, 0).unwrap().to_string(),
                record[8].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(4, 0, 0).unwrap().to_string(),
                record[9].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(5, 0, 0).unwrap().to_string(),
                record[10].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(6, 0, 0).unwrap().to_string(),
                record[11].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(7, 0, 0).unwrap().to_string(),
                record[12].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(8, 0, 0).unwrap().to_string(),
                record[13].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(9, 0, 0).unwrap().to_string(),
                record[14].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(10, 0, 0).unwrap().to_string(),
                record[15].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(11, 0, 0).unwrap().to_string(),
                record[16].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(12, 0, 0).unwrap().to_string(),
                record[17].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(13, 0, 0).unwrap().to_string(),
                record[18].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(14, 0, 0).unwrap().to_string(),
                record[19].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(15, 0, 0).unwrap().to_string(),
                record[20].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(16, 0, 0).unwrap().to_string(),
                record[21].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(17, 0, 0).unwrap().to_string(),
                record[22].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(18, 0, 0).unwrap().to_string(),
                record[23].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(19, 0, 0).unwrap().to_string(),
                record[24].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(20, 0, 0).unwrap().to_string(),
                record[25].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(21, 0, 0).unwrap().to_string(),
                record[26].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(22, 0, 0).unwrap().to_string(),
                record[27].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
        tx.execute(
            sql,
            params![
                date.and_hms_opt(23, 0, 0).unwrap().to_string(),
                record[28].parse::<f64>().unwrap() / 100.0 / 3.6 // 0.01MJ/m2 -> kWh/m2
            ],
        )?;
    }
    tx.commit()?;
    println!("Finished loading solar data. Total records: {}", counter);
    Ok(())
}
