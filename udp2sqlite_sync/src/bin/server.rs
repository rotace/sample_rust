use binrw::BinRead;
use rusqlite::Connection;
use std::io::Cursor;
use std::net::UdpSocket;
use udp2sqlite_sync::{
    MsgId,
    entity::{target, unit},
};

fn main() -> anyhow::Result<()> {
    // 古いデータベースファイルが存在する場合は削除
    if std::path::Path::new("./udb2sqlite.db").exists() {
        std::fs::remove_file("./udb2sqlite.db")?;
    }
    // SQLiteデータベース接続
    let db = Connection::open("udp2sqlite.db").unwrap();

    // マイグレーション
    unit::Dto::migration_up(&db)?;
    target::Dto::migration_up(&db)?;

    // UDPソケットのバッファ
    let mut buf = [0u8; 65535];

    // SQL用UDPソケット
    let sql_socket = UdpSocket::bind("0.0.0.0:3000")?;
    sql_socket.set_nonblocking(true)?;

    // バイナリ用UDPソケット
    let bin_socket = UdpSocket::bind("0.0.0.0:4000")?;
    bin_socket.set_nonblocking(true)?;

    loop {
        // SQL文受信
        if let Ok((len, _)) = sql_socket.recv_from(&mut buf) {
            let sql = String::from_utf8_lossy(&buf[..len]);
            println!("SQL受信: {}", sql);
            if let Err(e) = db.execute_batch(&sql) {
                eprintln!("SQL実行エラー: {}", e);
            }
        }

        // バイナリデータ受信
        if let Ok((len, _)) = bin_socket.recv_from(&mut buf) {
            let mut cursor = Cursor::new(&buf[..len]);
            // ヘッダの読み込み
            match MsgId::read_le(&mut cursor) {
                Ok(msg_id) => {
                    println!("ヘッダ受信: {:?}", msg_id);
                    // ヘッダに応じて適切な処理を行う
                    match msg_id {
                        MsgId::Unit => {
                            if let Ok(unit) = unit::Dto::read_le(&mut cursor) {
                                println!("ペイロード受信: {:?}", unit);
                                db.execute(
                                    "INSERT INTO units (id, value) VALUES (?1, ?2)",
                                    rusqlite::params![unit.id, unit.value],
                                )?;
                            } else {
                                eprintln!("バイナリデータの読み込みに失敗");
                            }
                        }
                        MsgId::Target => {
                            if let Ok(target) = target::Dto::read_le(&mut cursor) {
                                println!("ペイロード受信: {:?}", target);
                                db.execute(
                                    "INSERT INTO targets (id, value) VALUES (?1, ?2)",
                                    rusqlite::params![target.id, target.value],
                                )?;
                            } else {
                                eprintln!("バイナリデータの読み込みに失敗");
                            }
                        }
                        _ => eprintln!("未知のメッセージID: {:?}", msg_id),
                    }
                }
                Err(e) => eprintln!("ヘッダの読み込みに失敗: {}", e),
            }
        }

        // CPU負荷軽減
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
