use binrw::BinRead;
use binrw::BinWrite;

#[derive(BinRead, BinWrite, Debug)]
#[br(little)]
#[bw(little)]
pub struct MyData {
    pub id: i32,
    pub value: f64,
}