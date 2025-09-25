use clap::Parser;
use rusqlite::Connection;

#[derive(clap::Parser, Debug)]
#[command(name = "solar simulation cui")]
#[command(about = "A command line interface for solar simulation", long_about = None)]
struct Args {
    /// Input file path for power data
    #[arg(short, long, value_name = "FILE")]
    power_input: String,

    /// Input file path for solar data
    #[arg(short, long, value_name = "FILE")]
    solar_input: String,

    /// Input file path for weather data
    #[arg(short, long, value_name = "FILE")]
    weather_input: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // 古いデータベースファイルが存在する場合は削除
    if std::path::Path::new("data.sqlite").exists() {
        std::fs::remove_file("data.sqlite")?;
    }

    let mut conn = Connection::open("data.sqlite")?;
    solar_sim::load_parameter_data(&mut conn)?;
    solar_sim::load_power_data(&mut conn, &args.power_input)?;
    solar_sim::load_solar_data(&mut conn, &args.solar_input)?;
    solar_sim::load_weather_data(&mut conn, &args.weather_input)?;
    solar_sim::calc_simulation(&mut conn)?;
    Ok(())
}
