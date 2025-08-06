pub mod entity;

use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite)]
#[br(repr = u8, little)]
#[bw(repr = u8, little)]
pub enum MsgId {
    Target = 0x01,
    Unit = 0x02,
}
