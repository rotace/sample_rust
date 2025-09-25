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
    // 時別電力収支を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '時別電力収支' AS
        SELECT
            powers.timestamp,
            solars.'設計係数',
            solars.'太陽電池出力[m2]',
            solars.'太陽光発電量[kWh]',
            powers.'電力消費量[kWh]',
            (solars.'太陽光発電量[kWh]' - powers.'電力消費量[kWh]') AS '電力収支[kWh]'
        FROM
            '時別電力消費量' AS powers
        LEFT JOIN
            '時別太陽光発電量' AS solars
        ON
            powers.match_key = solars.match_key
        ORDER BY
            powers.timestamp",
        [],
    )?;
    // 日別電力収支を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '日別電力収支' AS
        WITH daily AS (
            SELECT
                date(timestamp) AS date,
                powers.'設計係数',
                powers.'太陽電池出力[m2]',
                SUM(powers.'電力消費量[kWh]') AS '電力消費量[kWh]',
                SUM(powers.'太陽光発電量[kWh]') AS '太陽光発電量[kWh]',
                SUM(MAX(0, powers.'電力収支[kWh]')) AS '余剰電力[kWh]',
                -SUM(MIN(0, powers.'電力収支[kWh]')) AS '不足電力[kWh]'
            FROM
                '時別電力収支' as powers
            GROUP BY
                date(timestamp),
                powers.'設計係数',
                powers.'太陽電池出力[m2]'
        ),
        tmp1 AS (
            SELECT
                date,
                daily.'設計係数',
                daily.'太陽電池出力[m2]',
                parameters.'蓄電池容量[kWh]',
                daily.'電力消費量[kWh]',
                daily.'太陽光発電量[kWh]',
                daily.'余剰電力[kWh]',
                daily.'不足電力[kWh]',
                MIN(daily.'余剰電力[kWh]', parameters.'蓄電池容量[kWh]') AS '電力蓄電量[kWh]'
            FROM
                daily
            CROSS JOIN
                '日別電力収支パラメータ' AS parameters
        ),
        tmp2 AS (
            SELECT
                *,
                MAX(0, tmp1.'不足電力[kWh]' - tmp1.'電力蓄電量[kWh]') AS '電力購入量[kWh]',
                MAX(0, tmp1.'電力蓄電量[kWh]' - tmp1.'不足電力[kWh]') AS '余剰蓄電量[kWh]'
            FROM
                tmp1
        ),
        tmp3 AS (
            SELECT
                *,
                COALESCE(tmp2.'電力蓄電量[kWh]' / tmp2.'蓄電池容量[kWh]', 0) * 100 AS '蓄電池使用率[%]',
                tmp2.'電力購入量[kWh]' / tmp2.'電力消費量[kWh]' * 100 AS '電力購入率[%]'
            FROM
                tmp2
        )
        SELECT
            *
        FROM
            tmp3
        ORDER BY
            date,
            '設計係数',
            '太陽電池出力[m2]',
            '蓄電池容量[kWh]'",
        [],
    )?;
    // 月別電力収支を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '月別電力収支' AS
        WITH monthly AS (
            SELECT
                date,
                strftime('%Y-%m', date) AS month,
                powers.'設計係数',
                powers.'太陽電池出力[m2]',
                powers.'蓄電池容量[kWh]',
                SUM(powers.'電力消費量[kWh]') AS '電力消費量[kWh]',
                SUM(powers.'太陽光発電量[kWh]') AS '太陽光発電量[kWh]',
                SUM(powers.'余剰電力[kWh]') AS '余剰電力[kWh]',
                SUM(powers.'不足電力[kWh]') AS '不足電力[kWh]',
                SUM(powers.'電力蓄電量[kWh]') AS '電力蓄電量[kWh]',
                SUM(powers.'電力購入量[kWh]') AS '電力購入量[kWh]',
                SUM(powers.'余剰蓄電量[kWh]') AS '余剰蓄電量[kWh]',
                AVG(powers.'蓄電池使用率[%]') AS '蓄電池使用率[%]',
                SUM(powers.'電力購入量[kWh]') / SUM(powers.'電力消費量[kWh]') * 100 AS '電力購入率[%]'
            FROM
                '日別電力収支' as powers
            GROUP BY
                strftime('%Y-%m', date),
                powers.'設計係数',
                powers.'太陽電池出力[m2]',
                powers.'蓄電池容量[kWh]'
        ),
        tmp1 AS (
            SELECT
                *,
                monthly.'電力消費量[kWh]' * 32 + 1400 AS '導入前電力料金(円)',
                monthly.'電力購入量[kWh]' * 32 + 1400 AS '導入後電力料金(円)'
            FROM
                monthly
        )
        SELECT
            *
        FROM
            tmp1
        ORDER BY
            month,
            '設計係数',
            '太陽電池出力[m2]',
            '蓄電池容量[kWh]'",
        [],
    )?;

    // 年別電力収支を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '年別電力収支' AS
        SELECT
            date,
            strftime('%Y', date) AS year,
            powers.'設計係数',
            powers.'太陽電池出力[m2]',
            powers.'蓄電池容量[kWh]',
            SUM(powers.'電力消費量[kWh]') AS '電力消費量[kWh]',
            SUM(powers.'太陽光発電量[kWh]') AS '太陽光発電量[kWh]',
            SUM(powers.'余剰電力[kWh]') AS '余剰電力[kWh]',
            SUM(powers.'不足電力[kWh]') AS '不足電力[kWh]',
            SUM(powers.'電力蓄電量[kWh]') AS '電力蓄電量[kWh]',
            SUM(powers.'電力購入量[kWh]') AS '電力購入量[kWh]',
            SUM(powers.'余剰蓄電量[kWh]') AS '余剰蓄電量[kWh]',
            AVG(powers.'蓄電池使用率[%]') AS '蓄電池使用率[%]',
            SUM(powers.'電力購入量[kWh]') / SUM(powers.'電力消費量[kWh]') * 100 AS '電力購入率[%]',
            SUM(powers.'導入前電力料金(円)') AS '導入前電力料金(円)',
            SUM(powers.'導入後電力料金(円)') AS '導入後電力料金(円)'
        FROM
            '月別電力収支' as powers
        GROUP BY
            strftime('%Y', date),
            powers.'設計係数',
            powers.'太陽電池出力[m2]',
            powers.'蓄電池容量[kWh]'
        ORDER BY
            year,
            '設計係数',
            '太陽電池出力[m2]',
            '蓄電池容量[kWh]'",
        [],
    )?;

    Ok(())
}
