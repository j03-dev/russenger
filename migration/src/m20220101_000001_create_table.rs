use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RussengerUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RussengerUser::SenderId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RussengerUser::Action).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RussengerUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RussengerUser {
    Table,
    SenderId,
    Action,
}
