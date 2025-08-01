use binrw::{BinRead, BinWrite};
use sea_orm::entity::prelude::*;

/// 通信データ兼テーブルデータ
#[derive(BinRead, BinWrite, Clone, Debug, PartialEq, DeriveEntityModel)]
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
