#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use tokio_util::io::ReaderStream;
use axum::{
    debug_handler,
    body::Body,
    extract::Multipart,
};
use std::path::PathBuf;
use tokio::{
    fs,
    io::AsyncWriteExt
};

use crate::models::_entities::{
    files::{ActiveModel, Entity, Column},
};

const UPLOAD_DIR: &str = "~/";

#[debug_handler]
pub async fn upload(
    _auth: auth::JWT,
    Path(articles_id): Path<i32>,
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Response> {
    let mut files = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!(error = ?err,"could not readd multipart");
        Error::BadRequest("could not readd multipartt".into())
    })? {
        let file_name = match field.file_name() {
            Some(file_name) => file_name.to_string(),
            _ => return Err(Error::BadRequest("file name not found".into())),
        };
        
        let content = field.bytes().await.map_err(|err| {
            tracing::error!(error = ?err,"could not readd bytes");
            Error::BadRequest("could not readd bytes".into())
        })?;

        let now = chrono::offset::Local::now()
            .format("%Y%m%d_%H%M%S")
            .to_string();
        let uuid = uuid::Uuid::new_v4().to_string();
        let folder = format!("{now}_{uuid}");
        let upload_folder = PathBuf::from(UPLOAD_DIR).join(&folder);
        fs::create_dir_all(&upload_folder).await?;

        let path = upload_folder.join(file_name);
        let mut f = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .await?;
        f.write_all(&content).await?;
        f.flush().await?;

        let file = ActiveModel {
            articles_id: ActiveValue::Set(articles_id),
            file_path: ActiveValue::Set(
                path.strip_prefix(UPLOAD_DIR)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            ),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await?;

        files.push(file)
    }

    format::json(files)
}

#[debug_handler]
pub async fn list(
    _auth: auth::JWT,
    Path(articles_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let files = Entity::find()
        .filter(Column::ArticlesId.eq(articles_id))
        .all(&ctx.db)
        .await?;

    format::json(files)
}

#[debug_handler]
pub async fn view(
    _auth: auth::JWT,
    Path(files_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let file = Entity::find_by_id(files_id)
        .one(&ctx.db)
        .await?
        .expect("File not found");

    let file = fs::File::open(format!("{UPLOAD_DIR}/{}", file.file_path)).await?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(format::render().response().body(body)?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("files")
        .add("/upload/:articles_id", post(upload))
        .add("/list/:articles_id", get(list))
        .add("/view/:files_id", get(view))
}