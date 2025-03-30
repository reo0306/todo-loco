#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;

use crate::models::_entities::articles::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Desirialize)]
pub struct Params {
    pub title: Option<String>,
    pub body: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.body = Set(self.body.clone());
    }
}

async fn load_item(ctx: &AppBody, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let res = articles::Entity::find().all(&ctx.db).await?;
    format::json(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/articles/")
        .add("/", get(list))
}
