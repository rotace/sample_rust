mod power;
mod solar;
mod weather;
pub use power::load_power_data;
pub use solar::load_solar_data;
pub use weather::load_weather_data;

use rusqlite::Connection;

/// シミュレーションを実行する
pub fn calc_simulation(conn: &mut Connection) -> anyhow::Result<()> {
    println!("Calculating simulation...");
    Ok(())
}
