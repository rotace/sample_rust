use binrw::BinRead;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, EntityTrait};
use std::io::Cursor;
use std::sync::Arc;
use tokio::net::UdpSocket;
use udp2sqlite_async::item;

async fn handle_sql(db: Arc<DatabaseConnection>, sql: &str) {
    // SQL文を実行
    let _ = db
        .execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            sql.to_string(),
        ))
        .await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Arc::new(Database::connect("sqlite://udp2sqlite.db").await?);

    // UDP3000: SQL文
    let sql_db = db.clone();
    let sql_task = tokio::spawn(async move {
        let socket = UdpSocket::bind("0.0.0.0:3000").await.unwrap();
        let mut buf = vec![0u8; 4096];
        loop {
            let (len, _) = socket.recv_from(&mut buf).await.unwrap();
            let sql = String::from_utf8_lossy(&buf[..len]);
            handle_sql(sql_db.clone(), &sql).await;
        }
    });

    // UDP4000: バイナリ
    let bin_db = db.clone();
    let bin_task: tokio::task::JoinHandle<Result<(), anyhow::Error>> = tokio::spawn(async move {
        let socket = UdpSocket::bind("0.0.0.0:4000").await.unwrap();
        let mut buf = vec![0u8; 4096];
        loop {
            let (len, _) = socket.recv_from(&mut buf).await.unwrap();
            let mut cursor = Cursor::new(&buf[..len]);
            let model : item::ActiveModel = item::Model::read_le(&mut cursor)?.into();
            // sea-ormでDB保存処理（例: INSERT文）
            item::Entity::insert_many([model])
                .exec(bin_db.as_ref())
                .await
                .unwrap();
        }
    });

    sql_task.await?;
    bin_task.await??;
    Ok(())
}
