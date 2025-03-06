use super::AuthResult;
use crate::auth::AuthService;
use crate::{CONFIG};
use async_trait::async_trait;

#[derive(Debug, Clone, Copy)]
pub struct TokenAuth;

#[async_trait]
impl AuthService for TokenAuth {
    type Error = ();
    type AuthKey = String;

    /// Authenticate a subdomain with an AuthKey
    async fn auth_sub_domain(
        &self,
        _auth_key: &Self::AuthKey,
        _subdomain: &str,
    ) -> Result<AuthResult, Self::Error> {
        if CONFIG.auth_token == *_auth_key {
            Ok(AuthResult::Available)
        } else {
            Ok(AuthResult::PaymentRequired)
        }
    }
}
