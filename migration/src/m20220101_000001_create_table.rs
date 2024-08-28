use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Schedule::Table)
                    .if_not_exists()
                    .col(uuid_uniq(Schedule::Id))
                    .col(string(Schedule::Title))
                    .col(date(Schedule::Day))
                    .col(timestamp_with_time_zone(Schedule::StartTime))
                    .col(timestamp_with_time_zone(Schedule::EndTime))
                    .col(string(Schedule::ActivityType))
                    .col(string(Schedule::Location))
                    .col(string(Schedule::Priority))
                    .col(string(Schedule::Notes))
                    .col(string(Schedule::Status))
                    .col(timestamp_with_time_zone(Schedule::AlertTime))
                    .col(string(Schedule::Duration))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Schedule::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Schedule {
    Table,
    Id,
    Title,
    Day,
    StartTime,
    EndTime,
    ActivityType,
    Location,
    Priority,
    Notes,
    Status,
    AlertTime,
    Duration,
}
