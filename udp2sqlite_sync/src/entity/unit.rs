use binrw::{BinRead, BinWrite};
use rusqlite::Connection;

#[derive(BinRead, BinWrite, Debug)]
#[brw(little)]
pub struct Dto {
    pub id: i32,
    pub value: f64,
}

impl Dto {
    pub fn migration_up(db: &Connection) -> anyhow::Result<()> {
        db.execute(
            "CREATE TABLE IF NOT EXISTS Units (
                id INTEGER PRIMARY KEY,
                value REAL NOT NULL
            )",
            [],
        )?;
        println!("Migration up: Created Units table");
        Ok(())
    }
}
