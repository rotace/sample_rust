use binrw::BinWrite;
use clap::{ArgAction, Parser, ValueEnum};
use std::io::{self, Cursor, Write};
use std::net::UdpSocket;
use udp2sqlite_async::MsgId;
use udp2sqlite_async::entity::{target, unit};

/// 利用可能なエンティティ
#[derive(ValueEnum, Clone, Debug)]
enum Entity {
    Unit,
    Target,
}

/// UDP client for SQL or binary send
#[derive(Parser, Debug)]
#[command(name = "client")]
#[command(about = "UDP client for SQL or binary send")]
struct Cli {
    /// Send SQL
    #[arg(short, long, action = ArgAction::SetTrue)]
    sql: bool,

    /// Send binary
    #[arg(short, long, value_enum, default_value_t = Entity::Unit)]
    entity: Entity,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.sql {
        // SQL送信
        let socket = UdpSocket::bind("0.0.0.0:0")?;
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
        let mut buf = Cursor::new(Vec::new());

        match cli.entity {
            Entity::Unit => {
                let unit = unit::Model {
                    id: 1,
                    value: 123.45,
                };
                MsgId::Unit.write(&mut buf).unwrap();
                unit.write(&mut buf).unwrap();
                println!("バイナリ送信: {:?}", unit);
            }
            Entity::Target => {
                let target = target::Dto {
                    id: 1,
                    value: 123.45,
                };
                MsgId::Target.write(&mut buf).unwrap();
                target.write(&mut buf).unwrap();
                println!("バイナリ送信: {:?}", target);
            }
        }
        socket.send_to(buf.get_ref(), server)?;
    }
    Ok(())
}
