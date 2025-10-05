use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Projects::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(
                        ColumnDef::new(Projects::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Projects::Description).text())
                    .col(ColumnDef::new(Projects::SubmissionDate).timestamp().not_null())
                    .col(
                        ColumnDef::new(Projects::HackathonId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Projects::Table, Projects::HackathonId)
                            .to(Hackathons::Table, Hackathons::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Projects::TeamId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Projects::Table, Projects::TeamId)
                            .to(HackathonTeams::Table, HackathonTeams::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
    Name,
    Slug,
    Description,
    TeamId,
    HackathonId,
    SubmissionDate,
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum HackathonTeams {
    Table,
    Id,
}
