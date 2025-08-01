use binrw::{BinRead, BinWrite};
use sea_orm::entity::prelude::*;

/// 通信データ
/// 
/// `Dto`は、UDP通信で送受信するデータの形式を定義する構造体で、`Model`との相互変換が可能です。
#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct Dto {
    pub id: u32,
    pub value: f64,
}

/// テーブルデータ
/// 
/// `Model`は、データベースのテーブルの形式を定義する構造体です。`Dto`との相互変換が可能です。
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

impl From<Model> for Dto {
    /// 不変テーブルデータから通信データに変換
    fn from(value: Model) -> Self {
        Dto {
            id: value.id,
            value: value.value,
        }
    }
}