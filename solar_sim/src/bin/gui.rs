use eframe::egui;
use rfd::FileDialog;
use rusqlite::Connection;

const DATABASE_FILE: &str = "data.sqlite";

struct MyApp {
    message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            message: "Please Push Button for Import CSV Files".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Solar System Simulator");

            if ui.button("Select Solar CSV").clicked() {
                if let Some(path) = FileDialog::new()
                    .set_title("Select Solar Data")
                    .add_filter("CSV", &["csv"])
                    .pick_file()
                {
                    let mut conn = match Connection::open(DATABASE_FILE) {
                        Ok(c) => c,
                        Err(e) => {
                            self.message = format!("Failed to open database: {}", e);
                            return;
                        }
                    };
                    match solar_sim::load_solar_data(&mut conn, path.to_str().unwrap()) {
                        Ok(_) => {
                            self.message = "Solar CSV loaded successfully".to_string();
                        }
                        Err(e) => {
                            self.message = format!("Failed to load Solar CSV: {}", e);
                        }
                    }
                }
            }

            if ui.button("Select Power CSV").clicked() {
                if let Some(path) = FileDialog::new()
                    .set_title("Select Power Data")
                    .add_filter("CSV", &["csv"])
                    .pick_file()
                {
                    let mut conn = match Connection::open(DATABASE_FILE) {
                        Ok(c) => c,
                        Err(e) => {
                            self.message = format!("Failed to open database: {}", e);
                            return;
                        }
                    };
                    match solar_sim::load_power_data(&mut conn, path.to_str().unwrap()) {
                        Ok(_) => {
                            self.message = "Power CSV loaded successfully".to_string();
                        }
                        Err(e) => {
                            self.message = format!("Failed to load Power CSV: {}", e);
                        }
                    }
                }
            }

            if ui.button("Selected Weather CSV").clicked() {
                if let Some(path) = FileDialog::new()
                    .set_title("Select Weather Data")
                    .add_filter("CSV", &["csv"])
                    .pick_file()
                {
                    let mut conn = match Connection::open(DATABASE_FILE) {
                        Ok(c) => c,
                        Err(e) => {
                            self.message = format!("Failed to open database: {}", e);
                            return;
                        }
                    };
                    match solar_sim::load_weather_data(&mut conn, path.to_str().unwrap()) {
                        Ok(_) => {
                            self.message = "Weather CSV loaded successfully".to_string();
                        }
                        Err(e) => {
                            self.message = format!("Failed to load Weather CSV: {}", e);
                        }
                    }
                }
            }

            ui.label(&self.message);
        });
    }
}

fn init() -> anyhow::Result<()> {
    // 古いデータベースファイルが存在する場合は削除
    if std::path::Path::new(DATABASE_FILE).exists() {
        std::fs::remove_file(DATABASE_FILE)?;
    }

    // データベースの初期化
    let mut conn = Connection::open(DATABASE_FILE)?;
    // パラメータデータの初期化
    solar_sim::load_parameter_data(&mut conn)?;
    // シミュレーションビューの初期化
    solar_sim::calc_simulation(&mut conn)?;

    Ok(())
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    init().expect("Failed to initialize the application");
    eframe::run_native(
        "Solar System Simulator",
        options,
        Box::new(|_| Ok(Box::new(MyApp::default()))),
    )
}
