use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Account::Table)
          .if_not_exists()
          .col(ColumnDef::new(Account::Id).big_integer().not_null().primary_key())
          .col(ColumnDef::new(Account::Phone).string().string_len(32).not_null())
          .col(ColumnDef::new(Account::Password).string().string_len(32).not_null())
          .col(ColumnDef::new(Account::Created).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
          .col(ColumnDef::new(Account::Updated).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
          .to_owned(),
      ).await;
    manager.create_table(
        Table::create()
          .table(Project::Table)
          .if_not_exists()
          .col(ColumnDef::new(Project::Id).big_integer().not_null().primary_key())
          .col(ColumnDef::new(Project::AppName).string().string_len(32).not_null())
          .col(ColumnDef::new(Project::BuildNumber).big_integer().not_null())
          .col(ColumnDef::new(Project::Created).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
          .col(ColumnDef::new(Project::Updated).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
          .to_owned(),
      ).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Project::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Account {
  Table,
  Id,
  Phone,
  Password,
  Created,
  Updated,
}

#[derive(DeriveIden)]
enum Project {
  Table,
  Id,
  AppName,
  BuildNumber,
  Created,
  Updated,
}
