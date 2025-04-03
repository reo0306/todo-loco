#[deub_handler]
pub async fn upload(
    _auth: auth::JWT,
    Path(articles_id): Path<i32>,
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Response> {
    let mut files = Vec::new();

    while let Some(field) =multipart.next_field().await.map_err(|err| {
        tracing::error!(error = ?err,"could not readd multipart");
        Error::BadRequest("could not readd multipartt".into())
    })? {
        let file_name = match field.file_name() {
            Some(file_name) => file_name.to_string(),
            _ => return Err(Error::BadRequest("file name not found".into())),
        };
        
        let cotent = field.bytes().await.map_err(|err| {
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

        let file = files::ActiveModel {
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
        .insert(&ctr.db)
        .await?;

        files.push(file)
    }

    format::json(files)
}