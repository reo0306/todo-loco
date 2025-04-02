use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

use super::m20250330_220123_articles::Articles;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(table_auto(Files::Table)
            .col(pk_auto(Files::Id))
            .col(integer(Files::ArticlesId))
            .col(string(Files::FilePath))
            .foreign_key(
                ForeignKey::create()
                    .name("FK_files_articles_id")
                    .from(Files::Table, Files::ArticlesId)
                    .to(Articles::Table, Articles::Id),
            )
            .to_owned(),
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "files").await
    }
}

#[derive(DeriveIden)]
pub enum Files {
    Table,
    Id,
    ArticlesId,
    FilePath,
}