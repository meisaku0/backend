use std::collections::HashSet;

use auth::jwt::JwtAuth;
use rocket::State;
use sea_orm::DatabaseConnection;

use crate::presentation::dto::sign_in::CredentialsDTO;

pub async fn action(credentials: CredentialsDTO, conn: &DatabaseConnection, jwt_auth: &State<JwtAuth>) -> String {
    let scopes = HashSet::from_iter(vec!["read".to_string(), "write".to_string()]);
    jwt_auth
        .generate_token("49a8579f-1718-4594-a8b6-84d6dd2654cb".to_string(), scopes, 3600)
        .unwrap()
}
