use binrw::BinRead;
use rusqlite::Connection;
use std::io::Cursor;
use std::net::UdpSocket;
use udp2sqlite_sync::entity::unit;

fn main() -> std::io::Result<()> {
    // SQL用UDPソケット
    let sql_socket = UdpSocket::bind("0.0.0.0:3000")?;
    sql_socket.set_nonblocking(true)?;

    // バイナリ用UDPソケット
    let bin_socket = UdpSocket::bind("0.0.0.0:4000")?;
    bin_socket.set_nonblocking(true)?;

    loop {
        // SQL文受信
        let mut buf = [0u8; 2048];
        if let Ok((size, _src)) = sql_socket.recv_from(&mut buf) {
            let sql = String::from_utf8_lossy(&buf[..size]);
            println!("SQL受信: {}", sql);
            // ここにデータベースに保存する処理を追加
        }

        // バイナリデータ受信
        let mut bin_buf = [0u8; 1024];
        if let Ok((size, _src)) = bin_socket.recv_from(&mut bin_buf) {
            let mut cursor = Cursor::new(&bin_buf[..size]);
            if let Ok(unit) = unit::Dto::read_le(&mut cursor) {
                println!("バイナリ受信: {:?}", unit);
                // ここにデータベースに保存する処理を追加
            } else {
                eprintln!("バイナリデータのパース失敗");
            }
        }

        // CPU負荷軽減
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
