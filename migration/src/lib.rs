#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250330_220123_articles;
mod m20250331_220935_comments;
mod m20250402_220935_files;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250330_220123_articles::Migration),
            Box::new(m20250331_220935_comments::Migration),
            Box::new(m20250402_220935_files::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}