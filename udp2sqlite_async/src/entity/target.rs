use binrw::{BinRead, BinWrite};
use sea_orm::entity::prelude::*;

/// 通信データ
#[derive(BinRead, BinWrite)]
#[brw(little)]
pub struct Dto {
    pub id: u32,
    pub value: f64,
}

impl From<Dto> for Model {
    /// 通信データから不変テーブルデータに変換
    fn from(value: Dto) -> Self {
        Model {
            id: value.id,
            value: value.value,
        }
    }
}

impl From<Dto> for ActiveModel {
    /// 通信データから可変テーブルデータに変換
    fn from(value: Dto) -> Self {
        let model: Model = value.into();
        model.into()
    }
}

/// テーブルデータ
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "targets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub value: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
