use binrw::BinRead;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, EntityTrait};
use sea_orm_migration::prelude::*;
use std::io::Cursor;
use std::sync::Arc;
use tokio::net::UdpSocket;
use udp2sqlite_async::{
    MsgId,
    entity::{target, unit},
    migration::Migrator,
};

/// SQL文を受信
async fn sql_server(db: Arc<DatabaseConnection>) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").await.unwrap();
    let mut buf = vec![0u8; 65535];
    loop {
        let (len, _) = socket.recv_from(&mut buf).await.unwrap();
        let sql = String::from_utf8_lossy(&buf[..len]);
        let _ = db
            .execute(sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                sql.to_string(),
            ))
            .await;
        println!("SQL受信: {}", sql);
    }
}

/// バイナリデータを受信
async fn bin_server(db: Arc<DatabaseConnection>) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:4000").await.unwrap();
    let mut buf = vec![0u8; 65535];
    loop {
        let (len, _) = socket.recv_from(&mut buf).await.unwrap();
        let mut cursor = Cursor::new(&buf[..len]);
        // ヘッダの読み込み
        match MsgId::read_le(&mut cursor) {
            Ok(msg_id) => {
                println!("ヘッダ受信: {:?}", msg_id);
                // ヘッダに応じて適切な処理を行う
                match msg_id {
                    MsgId::Unit => {
                        if let Ok(unit) = unit::Model::read_le(&mut cursor) {
                            println!("ペイロード受信: {:?}", unit);
                            unit::Entity::insert_many([unit::ActiveModel::from(unit)])
                                .exec(db.as_ref())
                                .await
                                .unwrap();
                        } else {
                            eprintln!("バイナリデータの読み込みに失敗");
                        }
                    }
                    MsgId::Target => {
                        if let Ok(target) = target::Dto::read_le(&mut cursor) {
                            println!("ペイロード受信: {:?}", target);
                            target::Entity::insert_many([target::ActiveModel::from(target)])
                                .exec(db.as_ref())
                                .await
                                .unwrap();
                        } else {
                            eprintln!("バイナリデータの読み込みに失敗");
                        }
                    }
                }
            }
            Err(e) => eprintln!("ヘッダの読み込みに失敗: {}", e),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 古いデータベースファイルが存在する場合は削除
    if std::path::Path::new("./udp2sqlite.db").exists() {
        std::fs::remove_file("./udp2sqlite.db")?;
    }
    // SQLiteデータベース接続
    let db = Arc::new(Database::connect("sqlite://./udp2sqlite.db?mode=rwc").await?);

    // マイグレーション
    Migrator::up(db.as_ref(), None).await?;

    let sql_task = tokio::spawn(sql_server(db.clone()));
    let bin_task = tokio::spawn(bin_server(db.clone()));

    sql_task.await??;
    bin_task.await??;
    Ok(())
}
