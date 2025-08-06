use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Units {
    Table,
    Id,
    Value,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Creating table: Units");
        manager
            .create_table(
                Table::create()
                    .table(Units::Table)
                    .if_not_exists()
                    .col(pk_auto(Units::Id))
                    .col(ColumnDef::new(Units::Value).double())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Units::Table).to_owned())
            .await
    }
}
