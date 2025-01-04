use std::collections::HashMap;

use minio::s3::utils::Multimap;
use rocket::form::Form;
use rocket::http::Status;
use rocket::tokio::io::AsyncReadExt;
use rocket::State;
use sea_orm::prelude::Uuid;
use sea_orm::DatabaseConnection;
use shared::responses::error::{AppError, Error};
use shared::storage::minio::MinioStorage;
use shared::wrappers::file::TempFileWrapper;

use crate::infrastructure::http::guards::auth::JwtGuard;

pub enum ChangeProfilePictureErrors {
    InvalidContentType,
    InvalidFileSize,
    ErrorReadingFile,
    ErrorProcessingFile,
    ErrorUploadingFile(String),
}

impl From<ChangeProfilePictureErrors> for Error {
    fn from(value: ChangeProfilePictureErrors) -> Self {
        match value {
            ChangeProfilePictureErrors::InvalidContentType => {
                AppError::BadRequest("Invalid content type".to_string()).into()
            },
            ChangeProfilePictureErrors::InvalidFileSize => AppError::BadRequest("Invalid file size".to_string()).into(),
            ChangeProfilePictureErrors::ErrorReadingFile => {
                AppError::InternalError("Error reading file".to_string()).into()
            },
            ChangeProfilePictureErrors::ErrorProcessingFile => {
                AppError::InternalError("Error processing file".to_string()).into()
            },
            ChangeProfilePictureErrors::ErrorUploadingFile(err) => AppError::InternalError(err).into(),
        }
    }
}

pub async fn action(
    conn: &DatabaseConnection, jwt_guard: JwtGuard, file: Form<TempFileWrapper<'_>>, minio: &State<MinioStorage>,
) -> Result<Status, Error> {
    let file_name = Uuid::new_v4().to_string();
    let file_extension = file
        .0
        .content_type()
        .and_then(|content_type| content_type.extension())
        .ok_or(ChangeProfilePictureErrors::InvalidContentType)?;
    let file_media_type = file
        .0
        .content_type()
        .map(|c| c.media_type())
        .ok_or(ChangeProfilePictureErrors::InvalidContentType)?;

    let file_name = format!("{}.{}", file_name, file_extension);
    let mut file_content = Vec::new();

    if (file.0.open().await.unwrap().read_to_end(&mut file_content).await).is_err() {
        return Err(ChangeProfilePictureErrors::ErrorReadingFile.into());
    }

    let mut tags: HashMap<String, String> = HashMap::new();
    tags.insert("type".to_string(), "profile_picture".to_string());

    let mut metadata = Multimap::new();
    metadata.insert("x-amz-meta-user-id".to_string(), jwt_guard.claims.sub.to_string());
    metadata.insert("x-amz-meta-file-name".to_string(), file.0.name().unwrap_or_default().to_string());

    let res = minio
        .upload_object(&file_name, file_content, file_media_type.to_string(), Some(tags), Some(metadata))
        .await
        .map_err(|e| ChangeProfilePictureErrors::ErrorUploadingFile(e.to_string()))?;

    Ok(Status::Created)
}
