use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "files")]
pub struct Model {
    pub create_at: DateTime,
    pub update_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub articles_id: i32,
    pub file_path: String,
}

#[derive(Copy, Cloen, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::files::Entity",
        from = "Column::ArticlesId",
        to = "super::articles::Column::Id"
    )]
    Articles,
}

impl Related<super::articles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Articles.def()
    }
}