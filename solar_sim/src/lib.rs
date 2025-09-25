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

    // 発電量を計算するためのビューを作成
    conn.execute(
        "CREATE VIEW IF NOT EXISTS '時別発電量' AS
        SELECT
            timestamp,
            match_key,
            solars.'傾斜面日射量[kWh/m2]',
            parameters.'太陽電池出力[m2]',
            parameters.'設計係数',
            solars.'傾斜面日射量[kWh/m2]' * parameters.'太陽電池出力[m2]' * parameters.'設計係数' AS '発電量[kWh]'
        FROM
            '時別傾斜面日射量' as solars
        CROSS JOIN
            '時別発電量パラメータ' as parameters
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
            solars.'発電量[kWh]',
            powers.'消費電力量[kWh]',
            (solars.'発電量[kWh]' - powers.'消費電力量[kWh]') AS '電力収支[kWh]'
        FROM
            '時別消費電力量' AS powers
        LEFT JOIN
            '時別発電量' AS solars
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
                SUM(powers.'発電量[kWh]') AS '発電量[kWh]',
                SUM(powers.'消費電力量[kWh]') AS '消費電力量[kWh]',
                SUM(MAX(0, powers.'電力収支[kWh]')) AS '余剰電力量[kWh]',
                SUM(MAX(0, -powers.'電力収支[kWh]')) AS '不足電力量[kWh]'
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
                daily.'発電量[kWh]',
                daily.'消費電力量[kWh]',
                daily.'余剰電力量[kWh]',
                daily.'不足電力量[kWh]',
                daily.'発電量[kWh]' + daily.'不足電力量[kWh]' - daily.'余剰電力量[kWh]' - daily.'消費電力量[kWh]' AS '漏れ量[kWh]',
                MIN(MIN(daily.'余剰電力量[kWh]', daily.'不足電力量[kWh]'), parameters.'蓄電池容量[kWh]') AS '蓄電量[kWh]'
            FROM
                daily
            CROSS JOIN
                '日別電力収支パラメータ' AS parameters
        ),
        tmp2 AS (
            SELECT
                *,
                MAX(0, tmp1.'不足電力量[kWh]' - tmp1.'蓄電量[kWh]') AS '買電量[kWh]',
                MAX(0, tmp1.'余剰電力量[kWh]' - tmp1.'蓄電量[kWh]') AS '売電量[kWh]'
            FROM
                tmp1
        ),
        tmp3 AS (
            SELECT
                *,
                COALESCE(tmp2.'蓄電量[kWh]' / tmp2.'蓄電池容量[kWh]', 0) * 100 AS '蓄電池使用率[%]',
                tmp2.'買電量[kWh]' / tmp2.'消費電力量[kWh]' * 100 AS '電力購入率[%]'
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
                SUM(powers.'発電量[kWh]') AS '発電量[kWh]',
                SUM(powers.'消費電力量[kWh]') AS '消費電力量[kWh]',
                SUM(powers.'余剰電力量[kWh]') AS '余剰電力量[kWh]',
                SUM(powers.'不足電力量[kWh]') AS '不足電力量[kWh]',
                SUM(powers.'蓄電量[kWh]') AS '蓄電量[kWh]',
                SUM(powers.'買電量[kWh]') AS '買電量[kWh]',
                SUM(powers.'売電量[kWh]') AS '売電量[kWh]',
                AVG(powers.'蓄電池使用率[%]') AS '蓄電池使用率[%]',
                SUM(powers.'買電量[kWh]') / SUM(powers.'消費電力量[kWh]') * 100 AS '電力購入率[%]'
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
                monthly.*,
                monthly.'消費電力量[kWh]' * 32 + 1400 AS '消費電力価格[円]',
                monthly.'買電量[kWh]' * 32 + 1400 AS '買電価格[円]',
                costs.'初期費用[円]'
            FROM
                monthly
            LEFT JOIN
                '太陽光発電初期費用' as costs
            ON
                monthly.'太陽電池出力[m2]' = costs.'太陽電池出力[m2]' AND
                monthly.'蓄電池容量[kWh]' = costs.'蓄電池容量[kWh]'
        ),
        tmp2 AS (
            SELECT
                *,
                CASE
                    WHEN tmp1.'消費電力価格[円]' - tmp1.'買電価格[円]' <= 0 THEN NULL
                    ELSE MAX(0, tmp1.'初期費用[円]' - 64 * tmp1.'売電量[kWh]') / (tmp1.'消費電力価格[円]' - tmp1.'買電価格[円]' + 8 * tmp1.'売電量[kWh]') / 12
                END AS '回収年数[年]'
            FROM
                tmp1
        )
        SELECT
            *
        FROM
            tmp2
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
        WITH yearly AS (
        SELECT
            date,
            strftime('%Y', date) AS year,
            powers.'設計係数',
            powers.'太陽電池出力[m2]',
            powers.'蓄電池容量[kWh]',
            SUM(powers.'発電量[kWh]') AS '発電量[kWh]',
            SUM(powers.'消費電力量[kWh]') AS '消費電力量[kWh]',
            SUM(powers.'余剰電力量[kWh]') AS '余剰電力量[kWh]',
            SUM(powers.'不足電力量[kWh]') AS '不足電力量[kWh]',
            SUM(powers.'蓄電量[kWh]') AS '蓄電量[kWh]',
            SUM(powers.'買電量[kWh]') AS '買電量[kWh]',
            SUM(powers.'売電量[kWh]') AS '売電量[kWh]',
            AVG(powers.'蓄電池使用率[%]') AS '蓄電池使用率[%]',
            SUM(powers.'買電量[kWh]') / SUM(powers.'消費電力量[kWh]') * 100 AS '電力購入率[%]',
            SUM(powers.'消費電力価格[円]') AS '消費電力価格[円]',
            SUM(powers.'買電価格[円]') AS '買電価格[円]',
            powers.'初期費用[円]'
        FROM
            '月別電力収支' as powers
        GROUP BY
            strftime('%Y', date),
            powers.'設計係数',
            powers.'太陽電池出力[m2]',
            powers.'蓄電池容量[kWh]'
        ),
        tmp1 AS (
            SELECT
                *,
                CASE
                    WHEN yearly.'消費電力価格[円]' - yearly.'買電価格[円]' <= 0 THEN NULL
                    ELSE MAX(0, yearly.'初期費用[円]' - 64 * yearly.'売電量[kWh]') / (yearly.'消費電力価格[円]' - yearly.'買電価格[円]' + 8 * yearly.'売電量[kWh]') / 12
                END AS '回収年数[年]'
            FROM
                yearly
        )
        SELECT
            *
        FROM
            tmp1
        ORDER BY
            year,
            '設計係数',
            '太陽電池出力[m2]',
            '蓄電池容量[kWh]'",
        [],
    )?;

    Ok(())
}
