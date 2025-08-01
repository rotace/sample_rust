use binrw::BinWrite;
use std::io::{self, Cursor, Write};
use std::net::UdpSocket;
use udp2sqlite_async::entity::unit;

fn main() -> anyhow::Result<()> {
    let is_sql = true;

    if is_sql {
        // SQL送信
        let socket = UdpSocket::bind("0.0.0.0:0")?; // 任意ポート
        let server = "127.0.0.1:3000";

        loop {
            print!("SQL入力: ");
            io::stdout().flush()?;
            let mut sql = String::new();
            io::stdin().read_line(&mut sql)?;
            socket.send_to(sql.trim().as_bytes(), server)?;
            println!("SQL送信: {}", sql.trim());
        }
    } else {
        // バイナリ送信
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let server = "127.0.0.1:4000";

        // 例としてMyData構造体を作成
        let unit = unit::Model {
            id: 1,
            value: 123.45,
        };
        let mut buf = Cursor::new(Vec::new());
        unit.write(&mut buf).unwrap();
        let bytes = buf.get_ref();
        socket.send_to(&bytes[..], server)?;
        println!("バイナリ送信: {:?}", unit);
    }
    Ok(())
}
