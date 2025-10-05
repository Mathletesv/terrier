use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HackathonTeams::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HackathonTeams::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(HackathonTeams::Name).string().not_null())
                    .col(
                        ColumnDef::new(HackathonTeams::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(HackathonTeams::HackathonId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(HackathonTeams::Table, HackathonTeams::HackathonId)
                            .to(Hackathons::Table, Hackathons::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(HackathonTeams::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(HackathonTeams::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HackathonTeams::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum HackathonTeams {
    Table,
    Id,
    Name,
    Slug,
    HackathonId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
}
