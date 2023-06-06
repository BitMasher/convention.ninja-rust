use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Users::AuthId).string().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Users::DeletedAt).date_time().null())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::DisplayName).string().null())
                    .col(ColumnDef::new(Users::Dob).date().not_null())
                    .col(ColumnDef::new(Users::ContactEmail).string().not_null())
                    .col(ColumnDef::new(Users::City).string().not_null())
                    .col(ColumnDef::new(Users::State).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Users::Table)
                    .name("idx-auth_id")
                    .if_not_exists()
                    .col(Users::AuthId).to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    AuthId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Name,
    DisplayName,
    Dob,
    ContactEmail,
    City,
    State,
}
