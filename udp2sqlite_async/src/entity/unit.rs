use binrw::{BinRead, BinWrite};
use sea_orm::entity::prelude::*;

/// 通信データ兼テーブルデータ
///
/// この構造体は、UdP通信で送受信するデータの形式の定義とデータベースのテーブルの定義を兼ねた構造体です。
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, BinRead, BinWrite)]
#[sea_orm(table_name = "units")]
#[brw(little)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub value: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
