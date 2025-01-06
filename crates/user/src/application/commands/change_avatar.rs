use std::cmp::min;
use std::collections::HashMap;
use std::io::Cursor;

use image::{GenericImageView, ImageFormat};
use minio::s3::response::PutObjectContentResponse;
use minio::s3::utils::Multimap;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::{MediaType, Status};
use rocket::tokio::io::AsyncReadExt;
use rocket::State;
use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter,
    TransactionTrait,
};
use shared::responses::error::{AppError, Error};
use shared::storage::minio::MinioStorage;

use crate::domain::entities::AvatarEntity;
use crate::domain::entities::AvatarEntity::Variant;
use crate::infrastructure::http::guards::auth::JwtGuard;
use crate::presentation::dto::change_avatar::ChangeAvatar;

pub enum ChangeProfilePictureErrors {
    InvalidContentType,
    InvalidFileSize,
    ErrorReadingFile,
    ErrorProcessingFile(String),
    ErrorUploadingFile(String),
    InvalidImageFormat,
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
            ChangeProfilePictureErrors::ErrorProcessingFile(err) => AppError::InternalError(err).into(),
            ChangeProfilePictureErrors::ErrorUploadingFile(err) => AppError::InternalError(err).into(),
            ChangeProfilePictureErrors::InvalidImageFormat => {
                AppError::BadRequest("Invalid image format".to_string()).into()
            },
        }
    }
}

async fn read_file_content(file: &TempFile<'_>) -> Result<Vec<u8>, ChangeProfilePictureErrors> {
    let mut file_content = Vec::new();
    file.open()
        .await
        .unwrap()
        .read_to_end(&mut file_content)
        .await
        .map_err(|_| ChangeProfilePictureErrors::ErrorReadingFile)?;
    Ok(file_content)
}

fn resize_image(img: &image::DynamicImage, width: u32, height: u32) -> Result<Vec<u8>, ChangeProfilePictureErrors> {
    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
    let mut bytes = Vec::new();
    resized
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png) // Cambia a formato según tu lógica
        .map_err(|e| ChangeProfilePictureErrors::ErrorProcessingFile(e.to_string()))?;
    Ok(bytes)
}

async fn delete_existing_avatars(user_id: &str, conn: &DatabaseTransaction) -> Result<(), Error> {
    AvatarEntity::Entity::delete_many()
        .filter(AvatarEntity::Column::UserId.eq(Uuid::parse_str(user_id).unwrap()))
        .exec(conn)
        .await?;
    Ok(())
}

async fn upload_variant(
    minio: &MinioStorage, variant_name: &str, image_data: Vec<u8>, media_type: &MediaType,
    tags: &HashMap<String, String>, metadata: &Multimap,
) -> Result<PutObjectContentResponse, ChangeProfilePictureErrors> {
    minio
        .upload_object(variant_name, image_data, media_type.to_string(), Some(tags.clone()), Some(metadata.clone()))
        .await
        .map_err(|e| ChangeProfilePictureErrors::ErrorUploadingFile(e.to_string()))
}

pub async fn action(
    conn: &DatabaseConnection, jwt_guard: JwtGuard, file: Form<ChangeAvatar<'_>>, minio: &State<MinioStorage>,
) -> Result<Status, Error> {
    let txn = conn.begin().await?;

    let file_content = read_file_content(&file.avatar.0).await?;
    let img = image::load_from_memory(&file_content).map_err(|_| ChangeProfilePictureErrors::InvalidImageFormat)?;

    let file_name = Uuid::new_v4().to_string();
    let extension = file.avatar
        .0
        .content_type()
        .and_then(|content_type| content_type.extension())
        .ok_or(ChangeProfilePictureErrors::InvalidContentType)?;

    let file_media_type = file.avatar
        .0
        .content_type()
        .map(|c| c.media_type())
        .ok_or(ChangeProfilePictureErrors::InvalidContentType)?;

    let variants = vec![
        (Variant::Original, None),
        (Variant::Thumbnail, Some((150, 150))),
        (Variant::Small, Some((300, 300))),
        (Variant::Medium, Some((600, 600))),
        (Variant::Large, Some((1200, 1200))),
    ];

    delete_existing_avatars(&jwt_guard.claims.sub, &txn).await?;

    let mut tags = HashMap::new();
    tags.insert("type".to_string(), "profile_picture".to_string());

    let mut metadata = Multimap::new();
    metadata.insert("x-amz-meta-user-id".to_string(), jwt_guard.claims.sub.to_string());

    let (original_width, original_height) = img.dimensions();

    for (variant, size) in variants {
        let image_data = if let Some((width, height)) = size {
            let new_width = min(width, original_width);
            let new_height = min(height, original_height);
            resize_image(&img, new_width, new_height)?
        } else {
            file_content.clone()
        };

        let variant_name = format!("{}-{}.{}", file_name, variant.to_string().to_lowercase(), extension);

        let upload = upload_variant(minio, &variant_name, image_data, file_media_type, &tags, &metadata).await?;

        let avatar = AvatarEntity::ActiveModel {
            user_id: ActiveValue::set(Uuid::parse_str(&jwt_guard.claims.sub).unwrap()),
            bucket_name: ActiveValue::set(upload.bucket_name),
            object_name: ActiveValue::set(upload.object_name.clone()),
            location: ActiveValue::set(upload.location.clone()),
            etag: ActiveValue::set(upload.etag),
            version_id: ActiveValue::set(Uuid::new_v4()),
            variant: ActiveValue::set(variant),
            ..Default::default()
        };

        avatar.insert(&txn).await?;
    }

    txn.commit().await?;
    Ok(Status::Created)
}
