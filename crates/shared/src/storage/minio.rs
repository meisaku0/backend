use std::collections::HashMap;

use config::AppConfig;
use minio::s3::args::{BucketExistsArgs, MakeBucketArgs};
use minio::s3::builders::ObjectContent;
use minio::s3::client::{Client, ClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::response::PutObjectContentResponse;
use minio::s3::utils::Multimap;

pub struct MinioStorage {
    pub client: Client,
    pub url: BaseUrl,
    pub bucket: String,
    pub provider: StaticProvider,
}

impl MinioStorage {
    pub async fn new(app_config: AppConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let url = app_config.minio_endpoint.unwrap().parse::<BaseUrl>()?;
        let provider = StaticProvider::new(
            &app_config.minio_access_key.unwrap_or_default(),
            &app_config.minio_secret_key.unwrap_or_default(),
            None,
        );

        let bucket = app_config.minio_bucket_name.unwrap_or_default();
        let client = ClientBuilder::new(url.clone())
            .provider(Some(Box::new(provider.clone())))
            .build()?;
        let bucket_exist = client.bucket_exists(&BucketExistsArgs::new(&bucket).unwrap()).await?;

        if !bucket_exist {
            client.make_bucket(&MakeBucketArgs::new(&bucket).unwrap()).await?;
        }

        Ok(MinioStorage {
            client,
            url,
            bucket,
            provider,
        })
    }

    pub async fn upload_object(
        &self, object_name: &str, data: Vec<u8>, content_type: String, tags: Option<HashMap<String, String>>,
        metadata: Option<Multimap>,
    ) -> Result<PutObjectContentResponse, Box<dyn std::error::Error + Send + Sync>> {
        let content = ObjectContent::from(data);

        let res = self
            .client
            .put_object_content(&self.bucket, object_name, content)
            .content_type(content_type)
            .tags(tags)
            .user_metadata(metadata)
            .send()
            .await?;

        Ok(res)
    }
}
