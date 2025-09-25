mod parameter;
mod power;
mod solar;
mod weather;
pub use parameter::load_parameter_data;
pub use power::load_power_data;
pub use solar::load_solar_data;
pub use weather::load_weather_data;

use rusqlite::Connection;

/// シミュレーションを実行する
pub fn calc_simulation(conn: &mut Connection) -> anyhow::Result<()> {
    println!("Calculating simulation...");

    // 太陽光発電量を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '時別太陽光発電量' AS
        SELECT
            timestamp,
            match_key,
            solars.'傾斜面日射量[kWh/m2]',
            parameters.'太陽電池出力[m2]',
            parameters.'設計係数',
            solars.'傾斜面日射量[kWh/m2]' * parameters.'太陽電池出力[m2]' * parameters.'設計係数' AS '太陽光発電量[kWh]'
        FROM
            '時別傾斜面日射量' as solars
        CROSS JOIN
            '時別太陽光発電量パラメータ' as parameters
        ORDER BY
            timestamp",
        [],
    )?;
    // 電力収支を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '時別電力収支' AS
        SELECT
            powers.timestamp,
            solars.'設計係数',
            solars.'太陽電池出力[m2]',
            solars.'太陽光発電量[kWh]',
            powers.'電力使用量[kWh]',
            (solars.'太陽光発電量[kWh]' - powers.'電力使用量[kWh]') AS '電力収支[kWh]'
        FROM
            '時別電力使用量' AS powers
        LEFT JOIN
            '時別太陽光発電量' AS solars
        ON
            powers.match_key = solars.match_key
        ORDER BY
            powers.timestamp",
        [],
    )?;
    // 日別蓄電量、日別電力購入量を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '日別電力収支' AS
        WITH daily_power AS (
            SELECT
                date(timestamp) AS date,
                powers.'設計係数',
                powers.'太陽電池出力[m2]',
                SUM(powers.'電力使用量[kWh]') AS '電力使用量[kWh]',
                SUM(powers.'太陽光発電量[kWh]') AS '太陽光発電量[kWh]',
                SUM(MAX(0, powers.'電力収支[kWh]')) AS '余剰電力[kWh]',
                -SUM(MIN(0, powers.'電力収支[kWh]')) AS '不足電力[kWh]'
            FROM
                '時別電力収支' as powers
            GROUP BY
                date(timestamp),
                powers.'設計係数',
                powers.'太陽電池出力[m2]'
        )
        SELECT
            daily_power.date,
            daily_power.'設計係数',
            daily_power.'太陽電池出力[m2]',
            parameters.'蓄電池容量[kWh]',
            daily_power.'余剰電力[kWh]',
            daily_power.'不足電力[kWh]',
            MIN(daily_power.'余剰電力[kWh]', parameters.'蓄電池容量[kWh]') AS '蓄電量[kWh]',
            daily_power.'不足電力[kWh]' - MIN(daily_power.'余剰電力[kWh]', parameters.'蓄電池容量[kWh]') AS '電力購入量[kWh]',
            daily_power.'電力使用量[kWh]',
            daily_power.'太陽光発電量[kWh]'
        FROM
            daily_power
        CROSS JOIN
            '日別電力収支パラメータ' AS parameters
        ORDER BY
            daily_power.date,
            daily_power.'設計係数',
            daily_power.'太陽電池出力[m2]',
            parameters.'蓄電池容量[kWh]'",
        [],
    )?;
    // 月別蓄電量、月別電力購入量、電力料金を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '月別電力収支' AS
        WITH monthly_power AS (
            SELECT
                strftime('%Y-%m', date) AS month,
                powers.'設計係数',
                powers.'太陽電池出力[m2]',
                powers.'蓄電池容量[kWh]',
                SUM(powers.'電力使用量[kWh]') AS '電力使用量[kWh]',
                SUM(powers.'太陽光発電量[kWh]') AS '太陽光発電量[kWh]',
                SUM(powers.'余剰電力[kWh]') AS '余剰電力[kWh]',
                SUM(powers.'不足電力[kWh]') AS '不足電力[kWh]',
                SUM(powers.'蓄電量[kWh]') AS '蓄電量[kWh]',
                SUM(powers.'電力購入量[kWh]') AS '電力購入量[kWh]'
            FROM
                '日別電力収支' as powers
            GROUP BY
                strftime('%Y-%m', date),
                powers.'設計係数',
                powers.'太陽電池出力[m2]',
                powers.'蓄電池容量[kWh]'
        )
        SELECT
            month AS '年月',
            monthly_power.'設計係数',
            monthly_power.'太陽電池出力[m2]',
            monthly_power.'蓄電池容量[kWh]',
            monthly_power.'電力使用量[kWh]',
            monthly_power.'太陽光発電量[kWh]',
            monthly_power.'余剰電力[kWh]',
            monthly_power.'不足電力[kWh]',
            monthly_power.'蓄電量[kWh]',
            monthly_power.'電力購入量[kWh]',
            monthly_power.'電力購入量[kWh]' * 32 + 1400 AS '月間電力料金(円)'
        FROM
            monthly_power
        ORDER BY
            month,
            '設計係数',
            '太陽電池出力[m2]',
            '蓄電池容量[kWh]'",
        [],
    )?;

    Ok(())
}
