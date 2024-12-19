use sea_orm::DatabaseConnection;

use crate::presentation::dto::sign_in::CredentialsDTO;

pub async fn action(credentials: CredentialsDTO, conn: &DatabaseConnection) {}
