use eframe::egui;
use rfd::FileDialog;
use rusqlite::{Connection, params};
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct SolarData {
    #[serde(rename = "日時")]
    timestamp: String,
    #[serde(rename = "日射量")]
    radiation: f64,
}

fn insert_solar_data(conn: &mut Connection, data: &[SolarData]) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS solar (timestamp TEXT, radiation REAL)",
        [],
    )?;

    let tx: rusqlite::Transaction<'_> = conn.transaction()?;
    {
        let mut stmt = tx.prepare("INSERT INTO solar (timestamp, radiation) VALUES (?1, ?2)")?;

        for record in data {
            stmt.execute(params![record.timestamp, record.radiation])?;
        }
    }

    tx.commit()
}

struct MyApp {
    message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            message: "CSVを選択してください".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Solar System Simulator");

            if ui.button("Select CSV").clicked() {
                if let Some(path) = FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
                    match File::open(&path) {
                        Ok(file) => {
                            let mut rdr = csv::Reader::from_reader(file);
                            let mut records = Vec::new();

                            for result in rdr.deserialize() {
                                match result {
                                    Ok(record) => records.push(record),
                                    Err(e) => {
                                        self.message = format!("Read CSV Error: {}", e);
                                        return;
                                    }
                                }
                            }

                            match Connection::open("data.db") {
                                Ok(mut conn) => match insert_solar_data(&mut conn, &records) {
                                    Ok(_) => self.message = "Written SQLite".to_string(),
                                    Err(e) => self.message = format!("Write DB Error: {}", e),
                                },
                                Err(e) => self.message = format!("Connect DB Error: {}", e),
                            }
                        }
                        Err(e) => self.message = format!("File Open Error: {}", e),
                    }
                }
            }

            ui.label(&self.message);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Solar System Simulator",
        options,
        Box::new(|_| Ok(Box::new(MyApp::default()))),
    )
}
