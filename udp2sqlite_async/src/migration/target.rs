use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Targets {
    Table,
    Id,
    Value,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Creating table: Targets");
        manager
            .create_table(
                Table::create()
                    .table(Targets::Table)
                    .if_not_exists()
                    .col(pk_auto(Targets::Id))
                    .col(ColumnDef::new(Targets::Value).double())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Targets::Table).to_owned())
            .await
    }
}
