use rusqlite::{Connection, params};

/// パラメータデータを読み込む
pub fn load_parameter_data(conn: &mut Connection) -> anyhow::Result<()> {
    println!("Loading parameta data...");

    // データベースにテーブルを作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS '時別発電量パラメータ' (
            '設計係数' REAL NOT NULL,
            '太陽電池出力[m2]' REAL NOT NULL
        )",
        [],
    )?;

    let design_factors = vec![0.55];
    let solar_outputs = vec![0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
    for &df in &design_factors {
        for &so in &solar_outputs {
            conn.execute(
                "INSERT INTO '時別発電量パラメータ' ('設計係数', '太陽電池出力[m2]') VALUES (?1, ?2)",
                params![df, so as f64],
            )?;
        }
    }

    // データベースにテーブルを作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS '日別電力収支パラメータ' (
            '蓄電池容量[kWh]' REAL NOT NULL
        )",
        [],
    )?;

    let battery_capacities = vec![0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
    for &bc in &battery_capacities {
        conn.execute(
            "INSERT INTO '日別電力収支パラメータ' ('蓄電池容量[kWh]') VALUES (?1)",
            params![bc],
        )?;
    }

    // データベースに太陽光発電の初期費用を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '太陽光発電初期費用' AS
        WITH
            tmp AS (
        SELECT
            t1.'太陽電池出力[m2]',
            t2.'蓄電池容量[kWh]',
            CASE WHEN t1.'太陽電池出力[m2]' == 0 THEN 0 ELSE t1.'太陽電池出力[m2]' * 250000 + 600000 END  AS '太陽電池初期費用[円]',
            CASE WHEN t2.'蓄電池容量[kWh]' == 0 THEN 0 ELSE t2.'蓄電池容量[kWh]' * 150000 + 1000000 END AS '蓄電池初期費用[円]'
        FROM
            '時別発電量パラメータ' as t1
        CROSS JOIN
            '日別電力収支パラメータ' as t2
        )
        SELECT
            *,
            tmp.'太陽電池初期費用[円]' + tmp.'蓄電池初期費用[円]' AS '初期費用[円]'
        FROM
            tmp",
        [],
    )?;

    println!("Finished loading parameter data.");
    Ok(())
}
