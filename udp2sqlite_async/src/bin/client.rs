use binrw::BinWrite;
use std::io::{self, Cursor, Write};
use std::net::UdpSocket;
use udp2sqlite_async::item;

fn main() -> io::Result<()> {
    // SQL送信
    let sql_socket = UdpSocket::bind("0.0.0.0:0")?; // 任意ポート
    let sql_server = "127.0.0.1:3000";

    print!("送信するSQL文を入力してください: ");
    io::stdout().flush()?;
    let mut sql = String::new();
    io::stdin().read_line(&mut sql)?;
    sql_socket.send_to(sql.trim().as_bytes(), sql_server)?;
    println!("SQL送信: {}", sql.trim());

    // バイナリ送信
    let bin_socket = UdpSocket::bind("0.0.0.0:0")?;
    let bin_server = "127.0.0.1:4000";

    // 例としてMyData構造体を作成
    let data = item::Model {
        id: 1,
        value: 123.45,
    };
    let mut buf = Cursor::new(Vec::new());
    data.write(&mut buf).unwrap();
    let bytes = buf.get_ref();
    bin_socket.send_to(&bytes[..], bin_server)?;
    println!("バイナリ送信: {:?}", data);

    Ok(())
}
