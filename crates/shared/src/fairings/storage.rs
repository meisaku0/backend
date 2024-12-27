use config::AppConfig;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::figment::Figment;
use rocket::{error_, info_, Build, Rocket};

use crate::storage::minio::MinioStorage;

pub struct Storage;

#[rocket::async_trait]
impl Fairing for Storage {
    fn info(&self) -> Info {
        Info {
            name: "Storage",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        // Extraer configuración
        let app_config = match Figment::extract::<AppConfig>(rocket.figment()) {
            Ok(config) => config,
            Err(err) => {
                error_!("Cannot extract AppConfig: {}", err);
                return Err(rocket);
            },
        };

        info_!("Initializing Minio storage...");

        let minio_s3 = match MinioStorage::new(app_config).await {
            Ok(storage) => {
                info_!("Minio storage initialized");
                storage
            },
            Err(err) => {
                error_!("Cannot initialize Minio storage: {}", err);
                return Err(rocket);
            },
        };

        Ok(rocket.manage(minio_s3))
    }
}
