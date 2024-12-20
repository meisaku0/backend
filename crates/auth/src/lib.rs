pub mod jwt;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use crate::jwt::JwtAuth;

    #[test]
    fn test_jwt_auth() {
        let auth = JwtAuth::new("mysecret".to_string());

        let scopes = HashSet::from_iter(vec!["read".to_string(), "write".to_string()]);
        let token = auth.generate_token("user1".to_string(), scopes.clone(), 3600).unwrap();

        let validated = auth.validate_token(&token).unwrap();
        assert_eq!(validated.claims.sub, "user1");

        auth.validate_scope(&token, "read").unwrap();
        assert!(auth.validate_scope(&token, "delete").is_err());

        auth.validate_multiple_scopes(&token, &["read", "write"]).unwrap();
        assert!(auth.validate_multiple_scopes(&token, &["read", "delete"]).is_err());
    }
}
