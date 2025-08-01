use binrw::{BinRead, BinWrite};

#[derive(BinRead, BinWrite, Debug)]
#[brw(little)]
pub struct Dto {
    pub id: i32,
    pub value: f64,
}
