use rusqlite::{Connection, params};

/// パラメータデータを読み込む
pub fn load_parameter_data(conn: &mut Connection) -> anyhow::Result<()> {
    println!("Loading parameta data...");

    // データベースにテーブルを作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS '時別太陽光発電量パラメータ' (
            '設計係数' REAL NOT NULL,
            '太陽電池出力[m2]' REAL NOT NULL
        )",
        [],
    )?;

    let design_factors = vec![0.55];
    let solar_outputs = vec![0., 2., 4., 6., 8.];
    for &df in &design_factors {
        for &so in &solar_outputs {
            conn.execute(
                "INSERT INTO '時別太陽光発電量パラメータ' ('設計係数', '太陽電池出力[m2]') VALUES (?1, ?2)",
                params![df, so],
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

    let battery_capacities = vec![0., 2., 4., 6., 8.];
    for &bc in &battery_capacities {
        conn.execute(
            "INSERT INTO '日別電力収支パラメータ' ('蓄電池容量[kWh]') VALUES (?1)",
            params![bc],
        )?;
    }
    println!("Finished loading parameter data.");
    Ok(())
}
